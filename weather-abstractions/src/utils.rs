use reqwest::{Error, Response, Url};
use serde::de::DeserializeOwned;

/// Parses the response from an HTTP request using the `reqwest` library.
///
/// The function takes a `reqwest::Response` as an argument and returns a `Result` that contains a nested `Result`.
/// If the HTTP status code of the response is successful (2xx), the inner `Result` will contain a value of the generic type `R` deserialized from the response body.
/// If the HTTP status code of the response is an error (not 2xx), the inner `Result` will contain a value of the generic type `E` deserialized from the response body.
/// If there is an error during the deserialization process, the function will return an `Error` variant.
///
/// # Arguments
///
/// * `res` - A `reqwest::Response` object representing the response from an HTTP request.
///
/// # Returns
///
/// A `Result` that contains a nested `Result`. If the HTTP status code of the response is successful (2xx),
/// the inner `Result` will contain a value of the generic type `R` deserialized from the response body.
/// If the HTTP status code of the response is an error (not 2xx), the inner `Result` will contain a value of the generic type `E` deserialized from the response body.
/// If there is an error during the deserialization process, the function will return an `Error` variant.
///
/// # Examples
///
/// ```
/// use reqwest::{Client, Error};
/// use serde::de::DeserializeOwned;
///
/// let client = Client::new();
/// let res = client
///     .get("https://jsonplaceholder.typicode.com/todos/1")
///     .send()?;
///
/// let parse_res = parse::<SuccessResponse, ErrorResponse>(res)?;
///
/// match parse_res {
///     Ok(success_response) => {
///         // Use success_response here
///     }
///     Err(error_response) => {
///         // Handle error_response here
///     }
/// }
/// ```
#[cfg(not(doctest))]
pub async fn parse<R, E>(res: Response) -> Result<Result<R, E>, Error>
where
    R: DeserializeOwned,
    E: DeserializeOwned,
{
    let code = res.status();
    if code.is_success() {
        let json = res.json::<R>().await;
        return match json {
            Ok(json) => Ok(Ok(json)),
            Err(err) => {
                //todo log
                Err(err)
            }
        };
    }

    match res.json::<E>().await {
        Ok(err_resp) => Ok(Err(err_resp)),
        Err(err) => {
            //todo log err.url()
            Err(err)
        }
    }
}

/// Builds an endpoint URL by combining the base URL and a given path.
///
/// # Arguments
///
/// * `base` - A reference to the base URL.
/// * `path` - A reference to a string that contains the endpoint path.
///
/// # Returns
///
/// A `Url` that represents the endpoint URL.
///
/// # Example
///
/// ```
/// use reqwest::Url;
/// use weather_abstractions::utils::build_endpoint;
///
/// let base = Url::parse("https://jsonplaceholder.typicode.com/").unwrap();
/// let path = "todos/1";
/// let endpoint = build_endpoint(&base, path);
///
/// assert_eq!(endpoint.as_str(), "https://jsonplaceholder.typicode.com/todos/1");
/// ```
pub fn build_endpoint(base: &Url, path: &str) -> Url {
    let mut endpoint = base.clone();
    endpoint.set_path(path);
    endpoint
}

/// Defines a set of functions that take no arguments and return a string.
/// The functions are generated based on the parameters passed to the macro.
///
/// # Examples
///
/// ```
/// use weather_abstractions::utils::generate_functions;
///
/// generate_functions!(
///     foo, "foo",
///     bar, "bar",
///     baz, "baz"
/// );
///
///
/// assert_eq!(foo(), "foo".to_string());
/// assert_eq!(bar(), "bar".to_string());
/// assert_eq!(baz(), "baz".to_string());
/// ```
#[cfg(not(doctest))]
#[macro_export]
macro_rules! generate_functions {
    ($($name:ident, $value:expr),*) => {
        $(
            pub fn $name() -> String {
                $value.to_string()
            }
        )*
    }
}

/// Asserts that the result of an expression matches a given pattern.
/// The macro will panic if the expression does not match the pattern.
///
/// # Examples
///
/// ```
/// let error = Error::NotFound;
///
/// assert_error!(error, Error::NotFound);
/// ```
#[cfg(not(doctest))]
#[macro_export]
macro_rules! assert_error {
    ($error:expr, $expected:pat) => {
        match $error {
            $expected => {}
            other => {
                panic!(
                    "error wasn't `{}` but got `{:?}`",
                    stringify!($expected),
                    other
                )
            }
        }
    };
}
