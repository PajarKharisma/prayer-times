use reqwest::Client;
use serde_json::Value;

const BASE_URL: &str = "https://equran.id/api/v2/shalat";

pub async fn fetch_provinces() -> Result<Vec<Value>, String> {
    let client = Client::builder()
        .use_rustls_tls()
        .build()
        .map_err(|e| e.to_string())?;

    let resp = client
        .get(format!("{}/provinsi", BASE_URL))
        .header("Accept", "application/json")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let json: Value = resp.json().await.map_err(|e| e.to_string())?;

    // API returns { data: [...] } or similar
    let data = json
        .get("data")
        .cloned()
        .unwrap_or(json);

    match data {
        Value::Array(arr) => Ok(arr),
        _ => Err("Unexpected response format for provinces".to_string()),
    }
}

pub async fn fetch_cities(province: &str) -> Result<Vec<Value>, String> {
    let client = Client::builder()
        .use_rustls_tls()
        .build()
        .map_err(|e| e.to_string())?;

    let body = serde_json::json!({ "provinsi": province });

    let resp = client
        .post(format!("{}/kabkota", BASE_URL))
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let json: Value = resp.json().await.map_err(|e| e.to_string())?;

    let data = json
        .get("data")
        .cloned()
        .unwrap_or(json);

    match data {
        Value::Array(arr) => Ok(arr),
        _ => Err("Unexpected response format for cities".to_string()),
    }
}

pub async fn fetch_monthly_schedule(
    province: &str,
    city: &str,
    month: u32,
    year: i32,
) -> Result<Vec<Value>, String> {
    let client = Client::builder()
        .use_rustls_tls()
        .build()
        .map_err(|e| e.to_string())?;

    let body = serde_json::json!({
        "provinsi": province,
        "kabkota": city,
        "bulan": month,
        "tahun": year,
    });

    let resp = client
        .post(BASE_URL)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    // DEBUG: print status + raw response text
    let _status = resp.status();
    let text = resp.text().await.map_err(|e| e.to_string())?;
    let json: Value = serde_json::from_str(&text).map_err(|e| format!("JSON parse error: {} | body: {}", e, &text[..text.len().min(200)]))?;

    // Response: { data: { jadwal: [...] } }
    let jadwal = json
        .get("data")
        .and_then(|d| d.get("jadwal"))
        .cloned()
        .unwrap_or_else(|| json.get("data").cloned().unwrap_or(json));

    match jadwal {
        Value::Array(arr) => Ok(arr),
        _ => Err("Unexpected response format for schedule".to_string()),
    }
}
