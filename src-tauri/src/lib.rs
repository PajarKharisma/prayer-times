mod api;
mod commands;
mod models;
mod timer;

use tauri::{
    image::Image,
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager, WebviewUrl, WebviewWindowBuilder,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_http::init())
        .setup(|app| {
            // Hide from dock (Accessory policy is set in tauri.conf.json, but also set here)
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            // Build tray icon
            let icon = load_tray_icon(app.handle());
            TrayIconBuilder::new()
                .icon(icon)
                .icon_as_template(true)
                .tooltip("Jadwal Sholat")
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        toggle_popup(app);
                    }
                })
                .build(app)?;

            // Start background prayer notification timer
            timer::start_prayer_timer(app.handle().clone());

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_provinces,
            commands::get_cities,
            commands::get_monthly_schedule,
            commands::get_settings,
            commands::save_settings,
            open_month_window,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn load_tray_icon(_app: &tauri::AppHandle) -> Image<'static> {
    // Use the bundled tray icon
    let icon_bytes = include_bytes!("../icons/tray-icon.png");
    Image::from_bytes(icon_bytes).expect("Failed to load tray icon")
}

fn toggle_popup(app: &tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        if window.is_visible().unwrap_or(false) {
            let _ = window.hide();
        } else {
            position_popup(&window);
            let _ = window.show();
            let _ = window.set_focus();
        }
    } else {
        // Create popup window
        let window = WebviewWindowBuilder::new(app, "main", WebviewUrl::App("index.html".into()))
            .title("Jadwal Sholat")
            .inner_size(320.0, 400.0)
            .resizable(false)
            .decorations(false)
            .transparent(true)
            .always_on_top(true)
            .skip_taskbar(true)
            .visible(false)
            .build()
            .expect("Failed to create popup window");

        position_popup(&window);
        let _ = window.show();
        let _ = window.set_focus();

        // Hide when focus lost
        let win_clone = window.clone();
        window.on_window_event(move |event| {
            if let tauri::WindowEvent::Focused(false) = event {
                let _ = win_clone.hide();
            }
        });
    }
}

fn position_popup(window: &tauri::WebviewWindow) {
    // Position near top-right (menu bar area)
    if let Ok(Some(monitor)) = window.primary_monitor() {
        let screen_size = monitor.size();
        let scale = monitor.scale_factor();
        let win_size = window.outer_size().unwrap_or(tauri::PhysicalSize::new(320, 420));

        // Place at top-right, below menu bar (~30px)
        let x = (screen_size.width as f64 / scale) as i32 - win_size.width as i32 - 10;
        let y = 30_i32;

        let _ = window.set_position(tauri::PhysicalPosition::new(
            (x as f64 * scale) as i32,
            (y as f64 * scale) as i32,
        ));
    }
}

#[tauri::command]
fn open_month_window(app: tauri::AppHandle) {
    if let Some(win) = app.get_webview_window("month") {
        let _ = win.show();
        let _ = win.set_focus();
        return;
    }
    let win = WebviewWindowBuilder::new(&app, "month", WebviewUrl::App("month.html".into()))
        .title("Jadwal Bulan Ini")
        .inner_size(420.0, 600.0)
        .resizable(true)
        .decorations(true)
        .always_on_top(false)
        .skip_taskbar(false)
        .build()
        .expect("Failed to create month window");
    let _ = win.show();
    let _ = win.set_focus();
}
