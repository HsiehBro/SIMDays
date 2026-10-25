use crate::state::{self, Status};
use crate::commands;
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

fn tooltip_for(st: &Status) -> String {
    match st.level.as_str() {
        "due" => "SIMDays：今天到期，请重置手机卡".into(),
        "overdue" => format!("SIMDays：已超期 {} 天", -st.days_left),
        "soon" => format!("SIMDays：{} 天后到期", st.days_left),
        _ => format!("SIMDays：还有 {} 天重置", st.days_left),
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
    let reset = MenuItem::with_id(app, "reset", "今天已重置", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
    let menu = Menu::with_items(
        app,
        &[&open, &reset, &PredefinedMenuItem::separator(app)?, &quit],
    )?;

    let st = state::current_status(app);
    let icon = icon_for(&st.level)?;

    TrayIconBuilder::with_id(TRAY_ID)
        .icon(icon)
        .menu(&menu)
        .show_menu_on_left_click(false)
        .tooltip(tooltip_for(&st))
        .on_menu_event(|app, event| match event.id.as_ref() {
            "open" => show_main(app),
            "reset" => {
                let _ = commands::do_reset(app);
            }
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

pub fn update(app: &AppHandle, st: &Status) {
    let Some(tray) = app.tray_by_id(TRAY_ID) else { return };
    if let Ok(icon) = icon_for(&st.level) {
        let _ = tray.set_icon(Some(icon));
    }
    let _ = tray.set_tooltip(Some(tooltip_for(st)));
}
