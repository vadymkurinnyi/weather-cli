use serde::Deserialize;

#[derive(Deserialize)]
pub struct CurrentResponse {
    pub location: Location,
    pub current: Current,
}

#[derive(Deserialize)]
pub struct Location {
    pub name: String,
    pub region: String,
    pub country: String,
    pub lat: f32,
    pub lon: f32,
}

#[derive(Deserialize)]
pub struct Current {
    pub last_updated: String,
    pub temp_c: f32,
    pub condition: Condition,
}

#[derive(Deserialize)]
pub struct Condition {
    pub text: String,
    pub icon: String,
    pub code: i64,
}

#[derive(Deserialize)]
pub struct HistoryResponse {
    pub location: Location,
    pub forecast: Forecast,
}

#[derive(Deserialize)]
pub struct ForecastResponse {
    pub location: Location,
    pub forecast: Forecast,
}

#[derive(Deserialize)]
pub struct Forecast {
    pub forecastday: Vec<ForecastDay>,
}
#[derive(Deserialize)]
pub struct ForecastDay {
    pub date: Option<String>,
    pub date_epoch: Option<i32>,
    pub day: Day,
}

#[derive(Deserialize)]
pub struct Day {
    pub avgtemp_c: f32,
    pub condition: Condition,
}
#[derive(Deserialize)]
pub struct FutureResponse {
    pub location: Location,
    pub forecast: Forecast,
}

#[derive(Deserialize)]
pub struct ErrorResponse {
    pub error: Error,
}
#[derive(Deserialize)]
pub struct Error {
    pub code: i32,
    pub message: String,
}
