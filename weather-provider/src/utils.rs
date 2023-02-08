use reqwest::Url;
use serde::de::DeserializeOwned;

pub async fn parse<R, E>(res: reqwest::Response) -> Result<Result<R, E>, reqwest::Error>
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

pub fn build_endpoint(base: &Url, path: &str) -> Url {
    let mut endpoint = base.clone();
    endpoint.set_path(path);
    endpoint
}

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
