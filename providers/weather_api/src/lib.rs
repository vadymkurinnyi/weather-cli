pub mod api_config;
pub mod builder;
mod error;
mod protocol;

use std::error::Error;

pub use api_config::PROVIDER_NAME;
use chrono::NaiveDate;
pub use error::WeatherApiError;
use protocol::*;
use reqwest_middleware::ClientWithMiddleware;
use weather_abstractions::*;

pub struct WeatherApi {
    api_key: String,
    endpoints: Endpoints,
    client: ClientWithMiddleware,
}
const MIN_FORECAS_DAYS: i64 = 1;
const MAX_FORECAS_DAYS: i64 = 14;
const MIN_FUTURE_DAYS: i64 = 15;
const MAX_FUTURE_DAYS: i64 = 300;

use async_trait::async_trait;
#[async_trait]
impl WeatherProvider for WeatherApi {
    async fn get_weather(
        &self,
        address: &str,
        date: Option<chrono::NaiveDate>,
    ) -> Result<Weather, Box<dyn Error + Send + Sync + 'static>> {
        Ok(Self::get_weather(self, address, date).await?)
    }
}
impl WeatherApi {
    async fn get_weather(
        &self,
        address: &str,
        date: Option<chrono::NaiveDate>,
    ) -> Result<Weather, WeatherApiError> {
        if let Some(date) = date {
            let today = chrono::offset::Utc::now().date_naive();
            let min_date = NaiveDate::from_ymd_opt(2010, 1, 1).expect("Date 2010-1-1 created");
            let dif_days = date.signed_duration_since(today).num_days();
            return match dif_days {
                0 => self.current(address).await,
                MIN_FORECAS_DAYS..=MAX_FORECAS_DAYS => self.forecast(address, dif_days).await,
                MIN_FUTURE_DAYS..=MAX_FUTURE_DAYS => self.future(address, date).await,
                _ if date < today && date > min_date => self.history(address, date).await,
                _ => Err(WeatherApiError::UnsupportedDate(date)),
            };
        }
        self.current(address).await
    }
    async fn current(&self, address: &str) -> Result<Weather, WeatherApiError> {
        let endpoint = self.endpoints.current.clone();
        let response = self
            .default_request_builder(&endpoint, address)
            .send()
            .await?;
        let resp = parse::<CurrentResponse>(response).await?;
        Ok(Weather::current(
            Temperature::from_c(resp.current.temp_c)?,
            resp.current.condition.text,
        ))
    }
    async fn history(
        &self,
        address: &str,
        date: chrono::NaiveDate,
    ) -> Result<Weather, WeatherApiError> {
        let endpoint = self.endpoints.history.clone();
        let dt = date.format("%Y-%m-%d").to_string();
        let response = self
            .default_request_builder(&endpoint, address)
            .query(&[("dt", &dt)])
            .send()
            .await?;
        let mut resp = parse::<HistoryResponse>(response).await?;
        let forecast = resp
            .forecast
            .forecastday
            .pop()
            .ok_or(WeatherApiError::JSON(
                self.endpoints.history.path().to_string(),
                "./forecast.forecastday".to_string(),
            ))?;
        Ok(Weather::history(
            Temperature::from_c(forecast.day.avgtemp_c)?,
            forecast.day.condition.text,
        ))
    }
    async fn forecast(&self, address: &str, day: i64) -> Result<Weather, WeatherApiError> {
        let endpoint = self.endpoints.forecast.clone();
        let response = self
            .default_request_builder(&endpoint, address)
            .query(&[
                ("days", day.to_string().as_str()),
                ("aqi", "no"),
                ("alerts", "no"),
            ])
            .send()
            .await?;
        let mut resp = parse::<ForecastResponse>(response).await?;
        let forecast = resp
            .forecast
            .forecastday
            .pop()
            .ok_or(WeatherApiError::JSON(
                endpoint.path().to_string(),
                "./forecast.forecastday".to_string(),
            ))?;

        Ok(Weather::forecast(
            Temperature::from_c(forecast.day.avgtemp_c)?,
            forecast.day.condition.text,
        ))
    }

    async fn future(&self, address: &str, date: NaiveDate) -> Result<Weather, WeatherApiError> {
        let endpoint = self.endpoints.future.clone();
        let dt = date.format("%Y-%m-%d").to_string();
        let response = self
            .default_request_builder(&endpoint, address)
            .query(&[("dt", &dt)])
            .send()
            .await?;
        let mut resp = parse::<FutureResponse>(response).await?;
        let forecast = resp
            .forecast
            .forecastday
            .pop()
            .ok_or(WeatherApiError::JSON(
                endpoint.path().to_string(),
                "./forecast.forecastday".to_string(),
            ))?;
        Ok(Weather::forecast(
            Temperature::from_c(forecast.day.avgtemp_c)?,
            forecast.day.condition.text,
        ))
    }

    fn default_request_builder(
        &self,
        endpoint: &Url,
        address: &str,
    ) -> reqwest_middleware::RequestBuilder {
        self.client
            .get(endpoint.clone())
            .query(&[("q", address), ("key", self.api_key.as_str())])
    }
}

async fn parse<T: DeserializeOwned>(res: reqwest::Response) -> Result<T, WeatherApiError> {
    let resp_or_error = crate::utils::parse::<T, ErrorResponse>(res)
        .await
        .map_err(reqwest_middleware::Error::Reqwest)?;
    resp_or_error.map_err(|e| WeatherApiError::Api(e.error.message, e.error.code as u16))
}

use self::api_config::Endpoints;
pub use builder::WeatherApiBuilder;
use reqwest::Url;
use serde::de::DeserializeOwned;

#[cfg(test)]
mod test {
    use self::api_config::ApiConfig;
    use super::*;
    use crate::assert_error;
    use rstest::rstest;
    use test_case::*;
    static SERVER_POOL: ServerPool = ServerPool::new(20);

    #[tokio::test]
    async fn get_weather_current_http_clien_deserialize_err() {
        let (cfg, server) = setup();

        let api_conf: ApiConfig = cfg.get(PROVIDER_NAME).expect("get config for WeatherApi");
        let path = api_conf.current_path;
        server.expect(
            Expectation::matching(request::method_path("GET", path.clone()))
                .respond_with(status_code(200).body(empty_json())),
        );

        let client = WeatherApiBuilder::build(&cfg).expect("WeatherApi created");

        let error = client
            .get_weather("London", None)
            .await
            .expect_err("weather result should be err");
        assert_error!(error, WeatherApiError::HttpClient(_));
    }

    #[tokio::test]
    async fn get_weather_current_api_err_from_server() {
        let (cfg, server) = setup();

        let api_conf: ApiConfig = cfg.get(PROVIDER_NAME).expect("get config for WeatherApi");
        let path = api_conf.current_path;
        let json = error_1008();
        let json = serde_json::to_string(&json).expect("serialize mock error json");
        server.expect(
            Expectation::matching(request::method_path("GET", path.clone()))
                .respond_with(status_code(400).body(json)),
        );

        let client = WeatherApiBuilder::build(&cfg).expect("WeatherApi created");
        let weather_result = client.get_weather("London", None).await;
        let error = weather_result.expect_err("weather result should be err");
        assert_error!(error, WeatherApiError::Api(_, _));
    }

    use config::Config;
    use httptest::{matchers::*, responders::*, Expectation, ServerHandle, ServerPool};
    fn setup<'a>() -> (Config, ServerHandle<'a>) {
        let api_key = "some-api-key";
        let server = SERVER_POOL.get_server();
        let base_url = server.url_str("");

        let conf = Config::builder()
            .set_override(format!("{PROVIDER_NAME}.apiKey"), api_key)
            .expect("Api key set")
            .set_override(format!("{PROVIDER_NAME}.baseUrl"), base_url)
            .expect("Test baseUrl key set")
            .build()
            .expect("config built");
        (conf, server)
    }

    #[rstest]
    #[case(date_plus_days(1095))]
    #[case(date_plus_days(365))]
    #[case(date_plus_days(301))]
    #[case(date(2009, 12, 31))]
    #[case(date(1970, 1, 1))]
    async fn get_weather_unsupported_date(#[case] date: Option<NaiveDate>) {
        let (cfg, _) = setup();
        let client = WeatherApiBuilder::build(&cfg).expect("WeatherApi created");

        let err = client
            .get_weather("Lviv", date)
            .await
            .expect_err("Unsupported date");
        assert_error!(err, WeatherApiError::UnsupportedDate(_))
    }

    #[rstest]
    #[case(
        "forecastPath",
        forecast_3c_rainy(),
        date_plus_days(10),
        temp_c(3.0),
        "Rainy"
    )]
    #[case(
        "historyPath",
        history_m_1c_snow(),
        date(2012, 1, 1),
        temp_c(-1.0),
        "Snow"
    )]
    #[case(
        "futurePath",
        future_12c_partly_cloudy(),
        date_plus_days(31),
        temp_c(12.0),
        "Partly cloudy"
    )]
    #[case("currentPath", current_8c_clear(), None, temp_c(8.0), "Clear")]
    #[tokio::test]
    async fn get_weather_success(
        #[case] path: &str,
        #[case] body: serde_json::Value,
        #[case] date: Option<NaiveDate>,
        #[case] expected_temp: Temperature,
        #[case] expected_condition: &str,
    ) {
        let (cfg, server) = setup();
        let endpoint = endpoint_from_config(&cfg, path);
        let json = serde_json::to_string(&body).expect("deserialize mock error json");
        server.expect(
            Expectation::matching(request::method_path("GET", endpoint.clone()))
                .respond_with(status_code(200).body(json)),
        );

        let client = WeatherApiBuilder::build(&cfg).expect("WeatherApi created");
        let weather = client
            .get_weather("Any address", date)
            .await
            .expect("weather result should be ok");

        assert_eq!(weather.temp, expected_temp);
        assert_eq!(weather.condition, expected_condition);
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

    fn temp_c(celsius: f32) -> Temperature {
        Temperature::from_c(celsius).expect("Temperature from celsius")
    }

    use std::collections::HashMap;
    fn endpoint_from_config(config: &Config, name: &str) -> String {
        let api_conf: ApiConfig = config
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

    mod test_case {
        use serde_json::{json, Value};

        pub fn forecast_3c_rainy() -> Value {
            json!({
                "location": {
                    "name": "Kyiv",
                    "region": "Kyiv",
                    "country": "Ukraine",
                    "lat": 50.45,
                    "lon": 30.52
                },
                "forecast": {
                    "forecastday": [
                        {
                            "date": "2023-03-05",
                            "date_epoch": 1615180800,
                            "day": {
                                "avgtemp_c": 3.0,
                                "condition": {
                                    "text": "Rainy",
                                    "icon": "https://www.example.com/rainy.png",
                                    "code": 1063
                                }
                            }
                        }
                    ]
                }
            }
            )
        }
        pub fn history_m_1c_snow() -> Value {
            json!({
                "location": {
                    "name": "New York",
                    "region": "New York",
                    "country": "United States",
                    "lat": 40.71,
                    "lon": -74.01
                },
                "forecast": {
                    "forecastday": [
                        {
                            "date": "2012-01-01",
                            "date_epoch": 1325376000,
                            "day": {
                                "avgtemp_c": -1.0,
                                "condition": {
                                    "text": "Snow",
                                    "icon": "https://www.example.com/snow.png",
                                    "code": 1066
                                }
                            }
                        }
                    ]
                }
            })
        }
        pub fn current_8c_clear() -> Value {
            json!(
                {
                    "location": {
                        "name": "London",
                        "region": "City of London, Greater London",
                        "country": "United Kingdom",
                        "lat": 51.52,
                        "lon": -0.11,
                        "tz_id": "Europe/London",
                        "localtime_epoch": 1675705322,
                        "localtime": "2023-02-06 17:42"
                    },
                    "current": {
                        "last_updated_epoch": 1675704600,
                        "last_updated": "2023-02-06 17:30",
                        "temp_c": 8.0,
                        "temp_f": 46.4,
                        "is_day": 0,
                        "condition": {
                            "text": "Clear",
                            "icon": "//cdn.weatherapi.com/weather/64x64/night/113.png",
                            "code": 1000
                        },
                        "wind_mph": 3.8,
                        "wind_kph": 6.1,
                        "wind_degree": 260,
                        "wind_dir": "W",
                        "pressure_mb": 1039.0,
                        "pressure_in": 30.68,
                        "precip_mm": 0.0,
                        "precip_in": 0.0,
                        "humidity": 49,
                        "cloud": 0,
                        "feelslike_c": 8.0,
                        "feelslike_f": 46.5,
                        "vis_km": 10.0,
                        "vis_miles": 6.0,
                        "uv": 1.0,
                        "gust_mph": 1.1,
                        "gust_kph": 1.8
                    }
                }
            )
        }
        pub fn future_12c_partly_cloudy() -> Value {
            json!({
                "location": {
                    "name": "New York",
                    "region": "New York",
                    "country": "United States",
                    "lat": 40.71,
                    "lon": -74.01
                },
                "forecast": {
                    "forecastday": [
                        {
                            "date": "2023-02-08",
                            "date_epoch": 1612585600,
                            "day": {
                                "avgtemp_c": 10.0,
                                "condition": {
                                    "text": "Sunny",
                                    "icon": "https://www.example.com/sunny.png",
                                    "code": 1000
                                }
                            }
                        },
                        {
                            "date": "2023-02-09",
                            "date_epoch": 1612672000,
                            "day": {
                                "avgtemp_c": 12.0,
                                "condition": {
                                    "text": "Partly cloudy",
                                    "icon": "https://www.example.com/partly_cloudy.png",
                                    "code": 1003
                                }
                            }
                        }
                    ]
                }
            }
            )
        }
        pub fn error_1008() -> Value {
            json!({"error":{"code":1008,"message":"API key is limited to get history data. Please check our pricing page and upgrade to higher plan."}})
        }
        pub fn empty_json() -> String {
            serde_json::to_string(&json!({})).expect("serialize {} json")
        }
    }
}
