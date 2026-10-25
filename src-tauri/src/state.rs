use chrono::{Duration, Local, NaiveDate};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

pub const DEFAULT_REMIND_BEFORE: i64 = 3;
pub const DEFAULT_ICON: &str = "📱";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub last_reset: NaiveDate,
    pub interval_days: i64,
    pub remind_before_days: i64,
    pub last_reminded: Option<NaiveDate>,
}

/// v1 单卡配置，只在读取旧 config.json 迁移时使用
#[derive(Debug, Clone, Deserialize)]
pub struct LegacyConfig {
    pub last_reset: NaiveDate,
    pub interval_days: i64,
    #[serde(default = "default_remind_before")]
    pub remind_before_days: i64,
    #[serde(default)]
    pub last_reminded: Option<NaiveDate>,
}

fn default_remind_before() -> i64 {
    DEFAULT_REMIND_BEFORE
}

fn default_view_mode() -> String {
    "list".to_string()
}

// deny_unknown_fields：防止 v1 单卡 JSON 被误解析成"空卡列表"而丢数据
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(default = "default_view_mode")]
    pub view_mode: String,
    #[serde(default)]
    pub cards: Vec<Card>,
}

impl Config {
    pub fn empty() -> Self {
        Self {
            view_mode: default_view_mode(),
            cards: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct CardStatus {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub today: NaiveDate,
    pub days_left: i64,
    pub next_reset: NaiveDate,
    pub last_reset: NaiveDate,
    pub interval_days: i64,
    pub remind_before_days: i64,
    /// normal | soon | due | overdue
    pub level: String,
}

/// 主窗口渲染用的整体状态：视图偏好 + 全部卡（按剩余天数升序，最紧急在前）
#[derive(Debug, Clone, Serialize)]
pub struct AppStatus {
    pub view_mode: String,
    pub cards: Vec<CardStatus>,
}

pub fn config_path(app: &AppHandle) -> Option<PathBuf> {
    app.path().app_data_dir().ok().map(|dir| dir.join("config.json"))
}

pub fn new_card_id() -> String {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    format!("card-{nanos:x}")
}

pub fn load(app: &AppHandle) -> Option<Config> {
    let path = config_path(app)?;
    let raw = fs::read_to_string(path).ok()?;
    if let Ok(cfg) = serde_json::from_str::<Config>(&raw) {
        return Some(cfg);
    }
    // v1 单卡格式：迁移成一张默认卡并立即落盘
    if let Ok(legacy) = serde_json::from_str::<LegacyConfig>(&raw) {
        let cfg = Config {
            view_mode: default_view_mode(),
            cards: vec![Card {
                id: new_card_id(),
                name: "手机卡".into(),
                icon: DEFAULT_ICON.into(),
                last_reset: legacy.last_reset,
                interval_days: legacy.interval_days,
                remind_before_days: legacy.remind_before_days,
                last_reminded: legacy.last_reminded,
            }],
        };
        let _ = save(app, &cfg);
        return Some(cfg);
    }
    None
}

pub fn load_or_default(app: &AppHandle) -> Config {
    load(app).unwrap_or_else(Config::empty)
}

pub fn save(app: &AppHandle, cfg: &Config) -> Result<(), String> {
    let path = config_path(app).ok_or_else(|| "无法定位配置目录".to_string())?;
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir).map_err(|e| format!("创建配置目录失败: {e}"))?;
    }
    let raw = serde_json::to_string_pretty(cfg).map_err(|e| e.to_string())?;
    fs::write(path, raw).map_err(|e| format!("写入配置失败: {e}"))
}

pub fn card_status(card: &Card, today: NaiveDate) -> CardStatus {
    let next_reset = card.last_reset + Duration::days(card.interval_days);
    let days_left = (next_reset - today).num_days();
    let level = if days_left < 0 {
        "overdue"
    } else if days_left == 0 {
        "due"
    } else if days_left <= card.remind_before_days {
        "soon"
    } else {
        "normal"
    };
    CardStatus {
        id: card.id.clone(),
        name: card.name.clone(),
        icon: card.icon.clone(),
        today,
        days_left,
        next_reset,
        last_reset: card.last_reset,
        interval_days: card.interval_days,
        remind_before_days: card.remind_before_days,
        level: level.to_string(),
    }
}

pub fn app_status(cfg: &Config, today: NaiveDate) -> AppStatus {
    let mut cards: Vec<CardStatus> = cfg.cards.iter().map(|c| card_status(c, today)).collect();
    cards.sort_by(|a, b| a.days_left.cmp(&b.days_left).then_with(|| a.name.cmp(&b.name)));
    AppStatus {
        view_mode: cfg.view_mode.clone(),
        cards,
    }
}

pub fn current_app_status(app: &AppHandle) -> AppStatus {
    let today = Local::now().date_naive();
    app_status(&load_or_default(app), today)
}

fn level_rank(level: &str) -> i32 {
    match level {
        "overdue" => 3,
        "due" => 2,
        "soon" => 1,
        _ => 0,
    }
}

/// 所有卡聚合：等级最差的一张（同级取剩余天数更少的），用于托盘图标与 tooltip
pub fn worst_status(statuses: &[CardStatus]) -> Option<CardStatus> {
    statuses
        .iter()
        .max_by_key(|s| (level_rank(&s.level), -s.days_left))
        .cloned()
}
