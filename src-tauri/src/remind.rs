use crate::state;
use chrono::Local;
use tauri::{AppHandle, Emitter, Manager};

/// 若处于临期/到期/超期状态且今天还没提醒过，显示置顶提醒窗。
/// 提醒窗在 tauri.conf.json 中预定义（启动时隐藏创建），这里只负责显示，
/// 避免运行时动态创建 webview 窗口导致白屏。
pub fn check_and_remind(app: &AppHandle) {
    let Some(cfg) = state::load(app) else { return };
    let today = Local::now().date_naive();
    let st = state::status_from(&cfg, today);
    if st.level == "normal" {
        return;
    }
    if cfg.last_reminded == Some(today) {
        return;
    }
    if show_remind_window(app).is_ok() {
        let mut cfg = cfg;
        cfg.last_reminded = Some(today);
        let _ = state::save(app, &cfg);
        let _ = app.emit("status-changed", &st);
    }
}

fn show_remind_window(app: &AppHandle) -> tauri::Result<()> {
    let win = app
        .get_webview_window("remind")
        .ok_or_else(|| tauri::Error::WindowNotFound)?;
    let _ = win.show();
    let _ = win.center();
    let _ = win.set_focus();
    Ok(())
}

/// "关闭"提醒窗 = 隐藏（窗口常驻，下次直接显示）
pub fn close_remind_window(app: &AppHandle) {
    if let Some(win) = app.get_webview_window("remind") {
        let _ = win.hide();
    }
}
