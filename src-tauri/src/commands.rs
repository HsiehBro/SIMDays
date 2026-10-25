use crate::state::{self, Config, Status};
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

fn broadcast(app: &AppHandle, st: &Status) -> Result<(), String> {
    tray::update(app, st);
    app.emit("status-changed", st).map_err(|e| e.to_string())
}

/// 命令跑在非主线程，而创建窗口必须在主线程，否则会得到未初始化的白框窗口
fn check_remind_main(app: &AppHandle) {
    let handle = app.clone();
    let _ = app.run_on_main_thread(move || remind::check_and_remind(&handle));
}

pub(crate) fn do_reset(app: &AppHandle) -> Result<Status, String> {
    let today = Local::now().date_naive();
    let mut cfg = state::load(app).ok_or_else(|| "尚未完成初始设置".to_string())?;
    cfg.last_reset = today;
    cfg.last_reminded = None;
    state::save(app, &cfg)?;
    let st = state::status_from(&cfg, today);
    broadcast(app, &st)?;
    remind::close_remind_window(app);
    Ok(st)
}

#[tauri::command]
pub fn get_status(app: AppHandle) -> Status {
    state::current_status(&app)
}

#[tauri::command]
pub fn setup(app: AppHandle, first_reset: String, interval_days: i64) -> Result<Status, String> {
    let first_reset =
        NaiveDate::parse_from_str(&first_reset, "%Y-%m-%d").map_err(|_| "日期格式不正确".to_string())?;
    let cfg = Config {
        last_reset: first_reset,
        interval_days: clamp_interval(interval_days),
        remind_before_days: state::DEFAULT_REMIND_BEFORE,
        last_reminded: None,
    };
    state::save(&app, &cfg)?;
    let st = state::current_status(&app);
    broadcast(&app, &st)?;
    check_remind_main(&app);
    Ok(st)
}

#[tauri::command]
pub fn reset_today(app: AppHandle) -> Result<Status, String> {
    do_reset(&app)
}

#[tauri::command]
pub fn save_settings(
    app: AppHandle,
    interval_days: i64,
    remind_before_days: i64,
    last_reset: Option<String>,
) -> Result<Status, String> {
    let mut cfg = state::load(&app).ok_or_else(|| "尚未完成初始设置".to_string())?;
    cfg.interval_days = clamp_interval(interval_days);
    cfg.remind_before_days = clamp_remind(remind_before_days);
    if let Some(s) = last_reset {
        cfg.last_reset = NaiveDate::parse_from_str(&s, "%Y-%m-%d")
            .map_err(|_| "日期格式不正确".to_string())?;
        // 日期变了，重置提醒状态，若新日期导致临期/超期则允许再次弹窗
        cfg.last_reminded = None;
    }
    state::save(&app, &cfg)?;
    let st = state::current_status(&app);
    broadcast(&app, &st)?;
    check_remind_main(&app);
    Ok(st)
}

#[tauri::command]
pub fn dismiss_reminder(app: AppHandle) -> Result<(), String> {
    let today = Local::now().date_naive();
    if let Some(mut cfg) = state::load(&app) {
        cfg.last_reminded = Some(today);
        state::save(&app, &cfg)?;
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
