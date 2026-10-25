use crate::state::{self, AppStatus, Card, CardStatus, Config};
use crate::{remind, tray};
use chrono::{Local, NaiveDate};
use tauri::{AppHandle, Emitter};
use tauri_plugin_autostart::ManagerExt as _;

fn clamp_interval(v: i64) -> i64 {
    v.clamp(1, 3650)
}

fn clamp_remind(v: i64) -> i64 {
    v.clamp(0, 30)
}

fn broadcast(app: &AppHandle, st: &AppStatus) -> Result<(), String> {
    tray::update(app, st);
    app.emit("cards-changed", st).map_err(|e| e.to_string())
}

/// 命令跑在非主线程，而创建窗口必须在主线程，否则会得到未初始化的白框窗口
fn check_remind_main(app: &AppHandle) {
    let handle = app.clone();
    let _ = app.run_on_main_thread(move || remind::check_and_remind(&handle));
}

fn valid_name(name: &str) -> Result<String, String> {
    let name = name.trim();
    if name.is_empty() {
        return Err("请填写卡名称".to_string());
    }
    if name.chars().count() > 20 {
        return Err("卡名称最长 20 个字".to_string());
    }
    Ok(name.to_string())
}

fn parse_date(s: &str) -> Result<NaiveDate, String> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|_| "日期格式不正确".to_string())
}

#[tauri::command]
pub fn get_cards(app: AppHandle) -> AppStatus {
    state::current_app_status(&app)
}

/// 提醒窗用：当前处于临期/到期/超期的卡（不管今天是否已提醒过）
#[tauri::command]
pub fn get_due_cards(app: AppHandle) -> Vec<CardStatus> {
    let today = Local::now().date_naive();
    state::app_status(&state::load_or_default(&app), today)
        .cards
        .into_iter()
        .filter(|c| c.level != "normal")
        .collect()
}

#[tauri::command]
pub fn add_card(
    app: AppHandle,
    name: String,
    icon: Option<String>,
    first_reset: String,
    interval_days: i64,
) -> Result<AppStatus, String> {
    let name = valid_name(&name)?;
    let first_reset = parse_date(&first_reset)?;
    let mut cfg: Config = state::load_or_default(&app);
    cfg.cards.push(Card {
        id: state::new_card_id(),
        name,
        icon: icon.unwrap_or_else(|| state::DEFAULT_ICON.to_string()),
        last_reset: first_reset,
        interval_days: clamp_interval(interval_days),
        remind_before_days: state::DEFAULT_REMIND_BEFORE,
        last_reminded: None,
    });
    state::save(&app, &cfg)?;
    let st = state::current_app_status(&app);
    broadcast(&app, &st)?;
    check_remind_main(&app);
    Ok(st)
}

#[tauri::command]
pub fn reset_card(app: AppHandle, card_id: String) -> Result<AppStatus, String> {
    let today = Local::now().date_naive();
    let mut cfg = state::load_or_default(&app);
    let card = cfg
        .cards
        .iter_mut()
        .find(|c| c.id == card_id)
        .ok_or_else(|| "找不到这张卡".to_string())?;
    card.last_reset = today;
    card.last_reminded = None;
    state::save(&app, &cfg)?;
    let st = state::current_app_status(&app);
    broadcast(&app, &st)?;
    Ok(st)
}

#[tauri::command]
pub fn save_card(
    app: AppHandle,
    card_id: String,
    name: String,
    icon: Option<String>,
    interval_days: i64,
    remind_before_days: i64,
    last_reset: Option<String>,
) -> Result<AppStatus, String> {
    let name = valid_name(&name)?;
    let mut cfg = state::load_or_default(&app);
    let card = cfg
        .cards
        .iter_mut()
        .find(|c| c.id == card_id)
        .ok_or_else(|| "找不到这张卡".to_string())?;
    card.name = name;
    if let Some(icon) = icon {
        card.icon = icon;
    }
    card.interval_days = clamp_interval(interval_days);
    card.remind_before_days = clamp_remind(remind_before_days);
    if let Some(s) = last_reset {
        card.last_reset = parse_date(&s)?;
        // 日期变了，重置提醒状态，若新日期导致临期/超期则允许再次弹窗
        card.last_reminded = None;
    }
    state::save(&app, &cfg)?;
    let st = state::current_app_status(&app);
    broadcast(&app, &st)?;
    check_remind_main(&app);
    Ok(st)
}

#[tauri::command]
pub fn delete_card(app: AppHandle, card_id: String) -> Result<AppStatus, String> {
    let mut cfg = state::load_or_default(&app);
    let before = cfg.cards.len();
    cfg.cards.retain(|c| c.id != card_id);
    if cfg.cards.len() == before {
        return Err("找不到这张卡".to_string());
    }
    state::save(&app, &cfg)?;
    let st = state::current_app_status(&app);
    broadcast(&app, &st)?;
    Ok(st)
}

#[tauri::command]
pub fn set_view_mode(app: AppHandle, mode: String) -> Result<(), String> {
    let mut cfg = state::load_or_default(&app);
    cfg.view_mode = if mode == "carousel" {
        "carousel".into()
    } else {
        "list".into()
    };
    state::save(&app, &cfg)
}

/// 「知道了」：把当前所有临期/到期/超期的卡记为今天已提醒，并关掉提醒窗
#[tauri::command]
pub fn dismiss_reminder(app: AppHandle) -> Result<(), String> {
    let today = Local::now().date_naive();
    if let Some(mut cfg) = state::load(&app) {
        let mut changed = false;
        for card in &mut cfg.cards {
            let st = state::card_status(card, today);
            if st.level != "normal" && card.last_reminded != Some(today) {
                card.last_reminded = Some(today);
                changed = true;
            }
        }
        if changed {
            state::save(&app, &cfg)?;
        }
    }
    remind::close_remind_window(&app);
    Ok(())
}

#[tauri::command]
pub fn get_autostart(app: AppHandle) -> bool {
    app.autolaunch().is_enabled().unwrap_or(false)
}

#[tauri::command]
pub fn set_autostart(app: AppHandle, enabled: bool) -> Result<(), String> {
    let autostart = app.autolaunch();
    let current = autostart.is_enabled().unwrap_or(false);
    if enabled == current {
        // 幂等：目标状态与当前一致时不动，避免 disable 不存在的自启项报 os error 2
        return Ok(());
    }
    if enabled {
        autostart.enable().map_err(|e| e.to_string())
    } else {
        autostart.disable().map_err(|e| e.to_string())
    }
}
