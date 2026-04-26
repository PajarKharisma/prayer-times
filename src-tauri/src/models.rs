#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Province {
    pub id: Option<serde_json::Value>,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct City {
    pub id: Option<serde_json::Value>,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PrayerTime {
    pub tanggal: String,
    pub imsak: String,
    pub subuh: String,
    pub terbit: String,
    pub dhuha: String,
    pub dzuhur: String,
    pub ashar: String,
    pub maghrib: String,
    pub isya: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub status: Option<String>,
    pub data: Option<T>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}
