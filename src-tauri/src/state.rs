use chrono::{Duration, Local, NaiveDate};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

pub const DEFAULT_INTERVAL: i64 = 180;
pub const DEFAULT_REMIND_BEFORE: i64 = 3;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub last_reset: NaiveDate,
    pub interval_days: i64,
    pub remind_before_days: i64,
    pub last_reminded: Option<NaiveDate>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Status {
    pub configured: bool,
    pub today: NaiveDate,
    pub days_left: i64,
    pub next_reset: NaiveDate,
    pub last_reset: NaiveDate,
    pub interval_days: i64,
    pub remind_before_days: i64,
    /// normal | soon | due | overdue
    pub level: String,
}

pub fn config_path(app: &AppHandle) -> Option<PathBuf> {
    app.path().app_data_dir().ok().map(|dir| dir.join("config.json"))
}

pub fn load(app: &AppHandle) -> Option<Config> {
    let path = config_path(app)?;
    let raw = fs::read_to_string(path).ok()?;
    serde_json::from_str(&raw).ok()
}

pub fn save(app: &AppHandle, cfg: &Config) -> Result<(), String> {
    let path = config_path(app).ok_or_else(|| "无法定位配置目录".to_string())?;
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir).map_err(|e| format!("创建配置目录失败: {e}"))?;
    }
    let raw = serde_json::to_string_pretty(cfg).map_err(|e| e.to_string())?;
    fs::write(path, raw).map_err(|e| format!("写入配置失败: {e}"))
}

pub fn status_from(cfg: &Config, today: NaiveDate) -> Status {
    let next_reset = cfg.last_reset + Duration::days(cfg.interval_days);
    let days_left = (next_reset - today).num_days();
    let level = if days_left < 0 {
        "overdue"
    } else if days_left == 0 {
        "due"
    } else if days_left <= cfg.remind_before_days {
        "soon"
    } else {
        "normal"
    };
    Status {
        configured: true,
        today,
        days_left,
        next_reset,
        last_reset: cfg.last_reset,
        interval_days: cfg.interval_days,
        remind_before_days: cfg.remind_before_days,
        level: level.to_string(),
    }
}

pub fn unconfigured_status() -> Status {
    let today = Local::now().date_naive();
    Status {
        configured: false,
        today,
        days_left: DEFAULT_INTERVAL,
        next_reset: today + Duration::days(DEFAULT_INTERVAL),
        last_reset: today,
        interval_days: DEFAULT_INTERVAL,
        remind_before_days: DEFAULT_REMIND_BEFORE,
        level: "normal".into(),
    }
}

pub fn current_status(app: &AppHandle) -> Status {
    match load(app) {
        Some(cfg) => status_from(&cfg, Local::now().date_naive()),
        None => unconfigured_status(),
    }
}
