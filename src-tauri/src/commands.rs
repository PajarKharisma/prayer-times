use crate::api;
use serde_json::Value;
use tauri_plugin_store::StoreExt;

// ─── Province ────────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn get_provinces(app: tauri::AppHandle) -> Result<Vec<Value>, String> {
    let store = app.store("pray-schedule-store.json").map_err(|e| e.to_string())?;

    // Cache: return if already stored
    if let Some(cached) = store.get("provinces") {
        if let Value::Array(arr) = cached {
            if !arr.is_empty() {
                return Ok(arr);
            }
        }
    }

    let provinces = api::fetch_provinces().await?;
    store.set("provinces", Value::Array(provinces.clone()));
    store.save().map_err(|e| e.to_string())?;

    Ok(provinces)
}

// ─── Cities ──────────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn get_cities(app: tauri::AppHandle, province: String) -> Result<Vec<Value>, String> {
    let store = app.store("pray-schedule-store.json").map_err(|e| e.to_string())?;

    let cache_key = format!("cities_{}", province);

    if let Some(cached) = store.get(&cache_key) {
        if let Value::Array(arr) = cached {
            if !arr.is_empty() {
                return Ok(arr);
            }
        }
    }

    let cities = api::fetch_cities(&province).await?;
    store.set(&cache_key, Value::Array(cities.clone()));
    store.save().map_err(|e| e.to_string())?;

    Ok(cities)
}

// ─── Monthly Schedule ────────────────────────────────────────────────────────

#[tauri::command]
pub async fn get_monthly_schedule(
    app: tauri::AppHandle,
    province: String,
    city: String,
    month: u32,
    year: i32,
) -> Result<Vec<Value>, String> {
    let store = app.store("pray-schedule-store.json").map_err(|e| e.to_string())?;

    let cache_key = format!("schedule_{}_{}_{:02}_{}", province, city, month, year);

    if let Some(cached) = store.get(&cache_key) {
        if let Value::Array(arr) = cached {
            if !arr.is_empty() {
                return Ok(arr);
            }
        }
    }

    let schedule = api::fetch_monthly_schedule(&province, &city, month, year).await?;
    store.set(&cache_key, Value::Array(schedule.clone()));
    store.save().map_err(|e| e.to_string())?;

    Ok(schedule)
}

// ─── Settings ────────────────────────────────────────────────────────────────

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Settings {
    pub province: Option<String>,
    pub city: Option<String>,
}

#[tauri::command]
pub async fn get_settings(app: tauri::AppHandle) -> Result<Settings, String> {
    let store = app.store("pray-schedule-store.json").map_err(|e| e.to_string())?;

    let province = store
        .get("selected_province")
        .and_then(|v| v.as_str().map(|s| s.to_string()));

    let city = store
        .get("selected_city")
        .and_then(|v| v.as_str().map(|s| s.to_string()));

    Ok(Settings { province, city })
}

#[tauri::command]
pub async fn save_settings(
    app: tauri::AppHandle,
    province: String,
    city: String,
) -> Result<(), String> {
    let store = app.store("pray-schedule-store.json").map_err(|e| e.to_string())?;

    store.set("selected_province", Value::String(province));
    store.set("selected_city", Value::String(city));
    store.save().map_err(|e| e.to_string())?;

    Ok(())
}
