use chrono::{Datelike, Local};
use serde_json::Value;
use tauri::AppHandle;
use tauri_plugin_notification::NotificationExt;
use tauri_plugin_store::StoreExt;

const PRAYER_KEYS: &[(&str, &str)] = &[
    ("subuh", "Subuh"),
    ("dzuhur", "Dzuhur"),
    ("ashar", "Ashar"),
    ("maghrib", "Maghrib"),
    ("isya", "Isya"),
];

pub fn start_prayer_timer(app: AppHandle) {
    std::thread::spawn(move || {
        loop {
            std::thread::sleep(std::time::Duration::from_secs(60));
            check_prayer_times(&app);
        }
    });
}

fn check_prayer_times(app: &AppHandle) {
    let now = Local::now();
    let current_time = now.format("%H:%M").to_string();

    // Load store synchronously by blocking on async
    let store = match app.store("prayer-times-store.json") {
        Ok(s) => s,
        Err(_) => return,
    };

    let province = store
        .get("selected_province")
        .and_then(|v| v.as_str().map(|s| s.to_string()));
    let city = store
        .get("selected_city")
        .and_then(|v| v.as_str().map(|s| s.to_string()));

    let (province, city) = match (province, city) {
        (Some(p), Some(c)) => (p, c),
        _ => return,
    };

    let month = now.format("%m").to_string().parse::<u32>().unwrap_or(1);
    let year = now.format("%Y").to_string().parse::<i32>().unwrap_or(2026);
    let cache_key = format!("schedule_{}_{}_{:02}_{}", province, city, month, year);

    let schedule = match store.get(&cache_key) {
        Some(Value::Array(arr)) => arr,
        _ => return,
    };

    let today_day = now.day();

    // tanggal is an integer in the API response (e.g. 26)
    let today_entry = schedule.iter().find(|entry| {
        entry
            .get("tanggal")
            .and_then(|v| v.as_u64())
            .map(|n| n as u32 == today_day)
            .unwrap_or(false)
    });

    let entry = match today_entry {
        Some(e) => e,
        None => return,
    };

    for (key, label) in PRAYER_KEYS {
        if let Some(prayer_time) = entry.get(*key).and_then(|v| v.as_str()) {
            // Normalize: "05:02" -> "05:02"
            let prayer_hm = prayer_time.trim();
            if prayer_hm == current_time {
                let _ = app
                    .notification()
                    .builder()
                    .title("Waktu Sholat")
                    .body(format!("Sudah masuk waktu {}", label))
                    .show();
            }
        }
    }
}
