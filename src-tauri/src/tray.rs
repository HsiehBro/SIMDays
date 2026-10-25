use crate::state::{self, AppStatus};
use tauri::{
    image::Image,
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager,
};

const TRAY_GREEN: &[u8] = include_bytes!("../icons/tray_green.png");
const TRAY_YELLOW: &[u8] = include_bytes!("../icons/tray_yellow.png");
const TRAY_RED: &[u8] = include_bytes!("../icons/tray_red.png");

const TRAY_ID: &str = "main-tray";

fn icon_for(level: &str) -> tauri::Result<Image<'static>> {
    let bytes = match level {
        "soon" | "due" => TRAY_YELLOW,
        "overdue" => TRAY_RED,
        _ => TRAY_GREEN,
    };
    Image::from_bytes(bytes).map(|img| img.to_owned())
}

/// 托盘只反映全局最差状态：有多张卡需要关注时，显示最紧急一张的信息
fn tooltip_for(st: &AppStatus) -> String {
    match state::worst_status(&st.cards) {
        None => "SIMDays：还没有添加手机卡".into(),
        Some(w) => {
            let tail = match w.level.as_str() {
                "due" => "今天到期，请重置".to_string(),
                "overdue" => format!("已超期 {} 天", -w.days_left),
                "soon" => format!("{} 天后到期", w.days_left),
                _ => format!("还有 {} 天重置", w.days_left),
            };
            format!("SIMDays · {}：{}", w.name, tail)
        }
    }
}

pub fn show_main(app: &AppHandle) {
    if let Some(win) = app.get_webview_window("main") {
        let _ = win.show();
        let _ = win.unminimize();
        let _ = win.set_focus();
    }
}

fn toggle_main(app: &AppHandle) {
    if let Some(win) = app.get_webview_window("main") {
        let visible = win.is_visible().unwrap_or(false);
        let focused = win.is_focused().unwrap_or(false);
        if visible && focused {
            let _ = win.hide();
        } else {
            show_main(app);
        }
    }
}

pub fn create(app: &AppHandle) -> tauri::Result<()> {
    let open = MenuItem::with_id(app, "open", "打开主界面", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&open, &PredefinedMenuItem::separator(app)?, &quit])?;

    let st = state::current_app_status(app);
    let level = state::worst_status(&st.cards).map(|w| w.level).unwrap_or_default();
    let icon = icon_for(&level)?;

    TrayIconBuilder::with_id(TRAY_ID)
        .icon(icon)
        .menu(&menu)
        .show_menu_on_left_click(false)
        .tooltip(tooltip_for(&st))
        .on_menu_event(|app, event| match event.id.as_ref() {
            "open" => show_main(app),
            "quit" => app.exit(0),
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                toggle_main(tray.app_handle());
            }
        })
        .build(app)?;
    Ok(())
}

pub fn update(app: &AppHandle, st: &AppStatus) {
    let Some(tray) = app.tray_by_id(TRAY_ID) else { return };
    let level = state::worst_status(&st.cards)
        .map(|w| w.level)
        .unwrap_or_default();
    if let Ok(icon) = icon_for(&level) {
        let _ = tray.set_icon(Some(icon));
    }
    let _ = tray.set_tooltip(Some(tooltip_for(st)));
}
