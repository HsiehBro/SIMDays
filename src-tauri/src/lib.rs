mod commands;
mod remind;
mod state;
mod tray;

use std::time::Duration;
use tauri::{Manager, WindowEvent};
use tauri_plugin_autostart::MacosLauncher;

#[cfg(target_os = "windows")]
fn set_pearl_title_bar(window: &tauri::WebviewWindow) {
    use std::{ffi::c_void, mem::size_of};

    #[link(name = "dwmapi")]
    extern "system" {
        fn DwmSetWindowAttribute(
            hwnd: *mut c_void,
            attribute: u32,
            value: *const c_void,
            value_size: u32,
        ) -> i32;
    }

    let _ = window.set_theme(Some(tauri::Theme::Light));

    let Ok(hwnd) = window.hwnd() else {
        return;
    };

    // DWM COLORREF uses 0x00BBGGRR. Caption colors are supported on Windows 11 22000+;
    // the per-window light theme above is the graceful fallback on older Windows versions.
    let attributes = [
        (20, 0_u32),                     // DWMWA_USE_IMMERSIVE_DARK_MODE = false
        (34, 0x00F4_E9_E0_u32),         // DWMWA_BORDER_COLOR: #E0E9F4
        (35, 0x00FC_F7_F4_u32),         // DWMWA_CAPTION_COLOR: #F4F7FC
        (36, 0x004B_36_29_u32),         // DWMWA_TEXT_COLOR: #29364B
    ];

    for (attribute, value) in attributes {
        // Unsupported DWM attributes simply leave the platform's theme in effect.
        unsafe {
            let _ = DwmSetWindowAttribute(
                hwnd.0 as *mut c_void,
                attribute,
                (&value as *const u32).cast(),
                size_of::<u32>() as u32,
            );
        }
    }
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            tray::show_main(app);
        }))
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
        .invoke_handler(tauri::generate_handler![
            commands::get_status,
            commands::setup,
            commands::reset_today,
            commands::save_settings,
            commands::dismiss_reminder,
            commands::get_autostart,
            commands::set_autostart,
        ])
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                match window.label() {
                    // 关闭主窗口 = 隐藏到托盘，退出走托盘菜单
                    "main" => {
                        let _ = window.hide();
                        api.prevent_close();
                    }
                    // 预定义的提醒窗不可销毁（Alt+F4 也只是隐藏）
                    "remind" => {
                        let _ = window.hide();
                        api.prevent_close();
                    }
                    _ => {}
                }
            }
        })
        .setup(|app| {
            tray::create(app.handle())?;

            #[cfg(target_os = "windows")]
            if let Some(window) = app.get_webview_window("main") {
                set_pearl_title_bar(&window);
            }

            // 启动时检查一次是否需要提醒
            let handle = app.handle().clone();
            let _ = handle.clone().run_on_main_thread(move || {
                remind::check_and_remind(&handle);
            });

            // 后台线程：每 30 分钟刷新一次状态与托盘，跨天/临期时自动提醒
            let handle = app.handle().clone();
            std::thread::spawn(move || loop {
                std::thread::sleep(Duration::from_secs(30 * 60));
                let h = handle.clone();
                let _ = h.clone().run_on_main_thread(move || {
                    remind::check_and_remind(&h);
                    let st = state::current_status(&h);
                    tray::update(&h, &st);
                });
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
