use crate::state::{self, CardStatus};
use chrono::Local;
use tauri::{AppHandle, Emitter, Manager};

/// 检查所有卡：只要有一张处于临期/到期/超期且今天还没提醒过，
/// 就弹出置顶提醒窗一次，并把这些卡记为今天已提醒。
/// 提醒窗在 tauri.conf.json 中预定义（启动时隐藏创建），这里只负责显示，
/// 避免运行时动态创建 webview 窗口导致白屏。
pub fn check_and_remind(app: &AppHandle) {
    let mut cfg = state::load_or_default(app);
    let today = Local::now().date_naive();

    let mut due: Vec<CardStatus> = Vec::new();
    let mut changed = false;
    for card in &mut cfg.cards {
        let st = state::card_status(card, today);
        if st.level != "normal" && card.last_reminded != Some(today) {
            card.last_reminded = Some(today);
            changed = true;
            due.push(st);
        }
    }
    if due.is_empty() {
        return;
    }
    if changed {
        let _ = state::save(app, &cfg);
    }
    if show_remind_window(app).is_ok() {
        // 提醒窗监听 cards-changed 渲染待处理列表；窗口尺寸由前端按行数自适应
        let st = state::current_app_status(app);
        let _ = app.emit("cards-changed", &st);
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
