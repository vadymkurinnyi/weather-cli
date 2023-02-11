mod api_config;
mod error;
mod protocol;

use std::error::Error;

use self::api_config::Endpoints;
pub use api_config::PROVIDER_NAME;
use chrono::{NaiveDate, NaiveDateTime};
use error::OpenWeatherError;
use protocol::*;
use reqwest::{Client, Url};
use weather_abstractions::*;

pub struct OpenWeatherMap {
    api_key: String,
    endpoints: Endpoints,
    client: Client,
}
const MIN_FORECAS_DAYS: i64 = 1;
const MAX_FORECAS_DAYS: i64 = 16;

use async_trait::async_trait;
use serde::de::DeserializeOwned;
#[async_trait]
impl WeatherProvider for OpenWeatherMap {
    async fn get_weather(
        &self,
        address: &str,
        date: Option<chrono::NaiveDate>,
    ) -> Result<Weather, Box<dyn Error + Send + Sync + 'static>> {
        Ok(Self::get_weather(self, address, date).await?)
    }
}
impl OpenWeatherMap {
    async fn get_weather(
        &self,
        address: &str,
        date: Option<NaiveDate>,
    ) -> Result<Weather, OpenWeatherError> {
        if let Some(date) = date {
            let today = chrono::offset::Utc::now().date_naive();
            let min_date = NaiveDate::from_ymd_opt(1979, 1, 1).expect("Date 1979-1-1 created");
            let diff_days = date.signed_duration_since(today).num_days();

            return match diff_days {
                MIN_FORECAS_DAYS..=MAX_FORECAS_DAYS => self.forecast(address, date).await,
                _ if date < today && date > min_date => self.history(address, date).await,
                _ => Err(OpenWeatherError::UnsupportedDate(date)),
            };
        }
        self.today(address).await
    }
    fn default_request_builder(&self, endpoint: &Url, address: &str) -> reqwest::RequestBuilder {
        self.client
            .get(endpoint.clone())
            .query(&[("q", address), ("appid", self.api_key.as_str())])
    }
    async fn today(&self, address: &str) -> Result<Weather, OpenWeatherError> {
        let endpoint = self.endpoints.weather.clone();

        let response = self
            .default_request_builder(&endpoint, address)
            .send()
            .await?;

        let mut resp = parse::<TodayResponse>(response).await?;

        let weather = resp
            .weather
            .pop()
            .ok_or_else(|| json_error(&endpoint, "weather"))?;
        Ok(Weather::current(
            Temperature::from_k(resp.main.temp)?,
            weather.main,
        ))
    }
    async fn history(&self, address: &str, date: NaiveDate) -> Result<Weather, OpenWeatherError> {
        let ts = NaiveDateTime::new(date, chrono::NaiveTime::default()).timestamp();
        let endpoint = self.endpoints.history.clone();
        let response = self
            .default_request_builder(&endpoint, address)
            .query(&[("start", ts.to_string().as_str()), ("cnt", "1")])
            .send()
            .await?;

        let mut resp = parse::<HistoryResponse>(response).await?;
        let mut histroy = resp
            .list
            .pop()
            .ok_or_else(|| json_error(&endpoint, "./list"))?;
        let weather = histroy
            .weather
            .pop()
            .ok_or_else(|| json_error(&endpoint, "./list/[0]/weather"))?;
        let temp = Temperature::from_k(histroy.main.temp)?;
        Ok(Weather::history(temp, weather.main))
    }
    async fn forecast(&self, address: &str, date: NaiveDate) -> Result<Weather, OpenWeatherError> {
        let ts = NaiveDateTime::new(date, chrono::NaiveTime::default()).timestamp();
        let endpoint = self.endpoints.forecast.clone();
        let response = self
            .default_request_builder(&endpoint, address)
            .query(&[("start", ts.to_string().as_str()), ("cnt", "1")])
            .send()
            .await?;

        let mut resp = parse::<ForecastResponse>(response).await?;
        let mut forecast = resp
            .list
            .pop()
            .ok_or_else(|| json_error(&endpoint, "./list"))?;
        let temp = forecast.temp.day;
        let temp = Temperature::from_k(temp)?;
        let weather = forecast
            .weather
            .pop()
            .ok_or_else(|| json_error(&endpoint, "./list/[0]/weather"))?;

        Ok(Weather::forecast(temp, weather.main))
    }
}

async fn parse<T: DeserializeOwned>(res: reqwest::Response) -> Result<T, OpenWeatherError> {
    let resp_or_error = crate::utils::parse::<T, ErrorResponse>(res).await?;
    resp_or_error.map_err(|e| OpenWeatherError::Api(e.message, e.cod as u16))
}
fn json_error(endpoint: &Url, path: impl Into<String>) -> OpenWeatherError {
    OpenWeatherError::Json(endpoint.to_string(), path.into())
}

#[cfg(test)]
mod tests {

    static SERVER_POOL: ServerPool = ServerPool::new(20);
    use super::*;
    use std::collections::HashMap;
    use test_case::*;

    #[rstest]
    #[case(
        "weatherPath",
        "baseUrl",
        current_298k_rain(),
        None,
        temp_k(298.48),
        "Rain"
    )]
    #[case(
        "forecastPath",
        "baseUrl",
        forecast_295k_rain(),
        date_plus_days(1),
        temp_k(295.76),
        "Rain"
    )]
    #[case(
        "forecastPath",
        "baseUrl",
        forecast_310k_clear(),
        date_plus_days(10),
        temp_k(310.11),
        "Clear"
    )]
    #[case(
        "historyPath",
        "historyBaseUrl",
        history_310k_clear(),
        date(1979, 1, 2),
        temp_k(320.75),
        "Sunny"
    )]
    #[tokio::test]
    async fn get_weather_success(
        #[case] path: &str,
        #[case] base_url: &str,
        #[case] body: serde_json::Value,
        #[case] date: Option<NaiveDate>,
        #[case] expected_temp: Temperature,
        #[case] expected_condition: &str,
    ) {
        let (cfg, server) = setup(base_url);
        let endpoint = endpoint_from_config(&cfg, path);
        let json = serde_json::to_string(&body).expect("deserialize mock error json");
        server.expect(
            Expectation::matching(request::method_path("GET", endpoint.clone()))
                .respond_with(status_code(200).body(json)),
        );

        let client = OpenWeatherMap::new(&cfg).expect("WeatherApi created");
        let weather = client
            .get_weather("Any address", date)
            .await
            .expect("weather result should be ok");

        assert_eq!(weather.temp, expected_temp);
        assert_eq!(weather.condition, expected_condition);
    }
    use config::Config;
    use httptest::{matchers::*, responders::*, Expectation, ServerHandle, ServerPool};
    use rstest::rstest;
    fn setup<'a>(base_url_conf_name: &str) -> (Config, ServerHandle<'a>) {
        let api_key = "some-api-key";
        let server = SERVER_POOL.get_server();
        let base_url = server.url_str("");

        let conf = Config::builder()
            .set_override(format!("{PROVIDER_NAME}.apiKey"), api_key)
            .expect("Api key set")
            .set_override(format!("{PROVIDER_NAME}.{base_url_conf_name}"), base_url)
            .expect("Test baseUrl key set")
            .build()
            .expect("config built");
        (conf, server)
    }
    #[rstest]
    #[case(date_plus_days(1095))]
    #[case(date_plus_days(365))]
    #[case(date_plus_days(17))]
    #[case(date(1978, 12, 31))]
    #[case(date(1970, 1, 1))]
    async fn get_weather_unsupported_date(#[case] date: Option<NaiveDate>) {
        let (cfg, _) = setup("baseUrl");
        let client = OpenWeatherMap::new(&cfg).expect("WeatherApi created");

        let err = client
            .get_weather("Ankara", date)
            .await
            .expect_err("Unsupported date");
        assert_error!(err, OpenWeatherError::UnsupportedDate(_))
    }

    fn endpoint_from_config(config: &Config, name: &str) -> String {
        let api_conf: crate::api_config::ApiConfig = config
            .get(PROVIDER_NAME)
            .expect("get config for WeatherApi");
        let json_conf = serde_json::to_string(&api_conf).expect("Serialize api conf");
        let conf_as_map: HashMap<String, String> =
            serde_json::from_str(&json_conf).expect("api conf as HashMap");
        conf_as_map
            .get(name)
            .expect("endpoint from config")
            .to_string()
    }
    fn date(year: i32, month: u32, day: u32) -> Option<NaiveDate> {
        NaiveDate::from_ymd_opt(year, month, day)
    }

    use chrono::offset::Utc;
    fn date_plus_days(days: u64) -> Option<NaiveDate> {
        Utc::now()
            .date_naive()
            .checked_add_days(chrono::Days::new(days))
    }

    fn temp_k(kelvin: f32) -> Temperature {
        Temperature::from_k(kelvin).expect("Temperature from kelvin")
    }
    mod test_case {
        use serde_json::{json, Value};
        pub fn current_298k_rain() -> Value {
            json!({
              "coord": {
                "lon": 10.99,
                "lat": 44.34
              },
              "weather": [
                {
                  "id": 501,
                  "main": "Rain",
                  "description": "moderate rain",
                  "icon": "10d"
                }
              ],
              "base": "stations",
              "main": {
                "temp": 298.48,
                "feels_like": 298.74,
                "temp_min": 297.56,
                "temp_max": 300.05,
                "pressure": 1015,
                "humidity": 64,
                "sea_level": 1015,
                "grnd_level": 933
              },
              "visibility": 10000,
              "wind": {
                "speed": 0.62,
                "deg": 349,
                "gust": 1.18
              },
              "rain": {
                "1h": 3.16
              },
              "clouds": {
                "all": 100
              },
              "dt": 1661870592,
              "sys": {
                "type": 2,
                "id": 2075663,
                "country": "IT",
                "sunrise": 1661834187,
                "sunset": 1661882248
              },
              "timezone": 7200,
              "id": 3163858,
              "name": "Zocca",
              "cod": 200
            })
        }
        pub fn forecast_295k_rain() -> Value {
            json!(
            {"city": {
                "id": 456789,
                "name": "London",
                "coord": {
                "lon": -0.13,
                "lat": 51.51
                },
                "country": "GB",
                "population": 8796000,
                "timezone": 3600
                },
                "cod": "200",
                "message": 0.02467,
                "cnt": 7,
                "list": [
                {
                "dt": 1561857200,
                "sunrise": 1561834187,
                "sunset": 1561882248,
                "temp": {
                "day": 279.66,
                "min": 288.93,
                "max": 279.66,
                "night": 280.31,
                "eve": 277.16,
                "morn": 288.93
                },
                "feels_like": {
                "day": 279.66,
                "night": 280.3,
                "eve": 277.1,
                "morn": 288.73
                },
                "pressure": 1017,
                "humidity": 56,
                "weather": [
                {
                "id": 800,
                "main": "Clear",
                "description": "clear sky",
                "icon": "01d"
                }
                ],
                "speed": 2.7,
                "deg": 209,
                "gust": 3.58,
                "clouds": 33,
                "pop": 0.7,
                "rain": 2.51
                },
                {
                "dt": 1561943600,
                "sunrise": 1561920656,
                "sunset": 1561968542,
                "temp": {
                "day": 295.76,
                "min": 287.73,
                "max": 295.76,
                "night": 289.37,
                "eve": 292.76,
                "morn": 287.73
                },
                "feels_like": {
                "day": 295.64,
                "night": 289.45,
                "eve": 292.97,
                "morn": 287.59
                },
                "pressure": 1014,
                "humidity": 60,
                "weather": [
                {
                "id": 500,
                "main": "Rain",
                "description": "light rain",
                "icon": "10d"
                }
                ],
                "speed": 2.29,
                "deg": 215,
                "gust": 3.27,
                "clouds": 66,
                "pop": 0.82,
                "rain": 5.32
                }
                          ]})
        }
        pub fn forecast_310k_clear() -> Value {
            json!(
            {
                "city": {
                  "id": 2540854,
                  "name": "Casablanca",
                  "coord": {
                    "lon": -7.62,
                    "lat": 33.6
                  },
                  "country": "MA",
                  "population": 3324000,
                  "timezone": 3600
                },
                "cod": "200",
                "message": 0.0318,
                "cnt": 7,
                "list": [
                  {
                    "dt": 1661857200,
                    "sunrise": 1661834187,
                    "sunset": 1661882248,
                    "temp": {
                      "day": 298.66,
                      "min": 288.93,
                      "max": 298.66,
                      "night": 290.31,
                      "eve": 297.16,
                      "morn": 288.93
                    },
                    "feels_like": {
                      "day": 298.66,
                      "night": 290.3,
                      "eve": 297.1,
                      "morn": 288.73
                    },
                    "pressure": 1017,
                    "humidity": 44,
                    "weather": [
                      {
                        "id": 800,
                        "main": "Clear",
                        "description": "clear sky",
                        "icon": "01d"
                      }
                    ],
                    "speed": 2.7,
                    "deg": 209,
                    "gust": 3.58,
                    "clouds": 53,
                    "pop": 0.7,
                    "rain": 0
                  },
                  {
                    "dt": 1661943600,
                    "sunrise": 1661920656,
                    "sunset": 1661968542,
                    "temp": {
                      "day": 310.11,
                      "min": 287.73,
                      "max": 310.11,
                      "night": 289.37,
                      "eve": 292.76,
                      "morn": 287.73
                    },
                    "feels_like": {
                      "day": 295.64,
                      "night": 289.45,
                      "eve": 292.97,
                      "morn": 287.59
                    },
                    "pressure": 1014,
                    "humidity": 60,
                    "weather": [
                      {
                        "id": 800,
                        "main": "Clear",
                        "description": "clear sky",
                        "icon": "01d"
                      }
                    ],
                    "speed": 2.29,
                    "deg": 215,
                    "gust": 3.27,
                    "clouds": 66,
                    "pop": 0.82,
                    "rain": 0
                  }
                      ]})
        }
        pub fn history_310k_clear() -> Value {
            json!(
            {
                "message": "Count: 24",
                "cod": "200",
                "city_id": 4298960,
                "calctime": 0.00297316,
                "cnt": 1,
                "list": [
                {
                "dt": 1578384000,
                "main": {
                  "temp": 320.75,
                  "feels_like": 340.0,
                  "pressure": 1014,
                  "humidity": 74,
                  "temp_min": 299.73,
                  "temp_max": 320.75
                },
                "wind": {
                  "speed": 2.16,
                  "deg": 87
                },
                "clouds": {
                  "all": 90
                },
                "weather": [
                  {
                    "id": 501,
                    "main": "Sunny",
                    "description": "sunny",
                    "icon": "10n"
                  }
                ],
                "rain": {
                  "1h": 0.9
                }
             },
                  ]})
        }
    }
}
