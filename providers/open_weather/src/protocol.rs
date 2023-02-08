#[derive(Deserialize)]
pub struct ErrorResponse {
    pub cod: i32,
    pub message: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TodayResponse {
    pub coord: Coord,
    pub weather: Vec<TheWeather>,
    pub base: String,
    pub main: Main,
    pub visibility: i64,
    pub wind: Wind,
    pub clouds: Clouds,
    pub dt: i64,
    pub sys: Option<Sys>,
    pub timezone: i64,
    pub id: i64,
    pub name: String,
    pub cod: i64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Coord {
    pub lon: f32,
    pub lat: f32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TheWeather {
    pub id: i64,
    pub main: String,
    pub description: String,
    pub icon: String,
}

use serde::Deserialize;
#[derive(Deserialize)]
pub struct Main {
    pub temp: f32,
    pub feels_like: f32,
    pub temp_min: f32,
    pub temp_max: f32,
    pub pressure: i64,
    pub humidity: i64,
}

#[derive(Deserialize)]
pub struct Wind {
    pub speed: f32,
    pub deg: i64,
}

#[derive(Deserialize)]
pub struct Clouds {
    pub all: i64,
}

#[derive(Deserialize)]
pub struct Sys {
    #[serde(rename = "type")]
    pub type_field: Option<i64>,
    pub id: Option<i64>,
    pub country: String,
    pub sunrise: i64,
    pub sunset: i64,
}

#[derive(Deserialize)]
pub struct HistoryResponse {
    pub cnt: i8,
    pub list: Vec<HistoryTs>,
}

#[derive(Deserialize)]
pub struct HistoryTs {
    pub main: Main,
    pub wind: Wind,
    pub clouds: Clouds,
    pub weather: Vec<TheWeather>,
    pub dt: i64,
}
#[derive(Deserialize)]
pub struct List {
    pub dt: i64,
    pub sunrise: i64,
    pub sunset: i64,
    pub temp: Temp,
    pub feels_like: FeelsLike,
    pub pressure: i64,
    pub humidity: i64,
    pub weather: Vec<TheWeather>,
    pub speed: f32,
    pub deg: i64,
    pub gust: f32,
    pub clouds: i64,
    pub pop: f32,
    pub rain: f32,
}
#[derive(Deserialize)]
pub struct Temp {
    pub day: f32,
    pub min: f32,
    pub max: f32,
    pub night: f32,
    pub eve: f32,
    pub morn: f32,
}
#[derive(Deserialize)]

pub struct FeelsLike {
    pub day: f32,
    pub night: f32,
    pub eve: f32,
    pub morn: f32,
}

#[derive(Deserialize)]
pub struct ForecastResponse {
    pub cnt: i8,
    pub list: Vec<List>,
}
