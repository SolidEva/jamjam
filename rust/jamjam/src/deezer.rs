use anyhow::{anyhow, Context, Result};
use reqwest::header::{HeaderMap, HeaderValue, COOKIE, USER_AGENT};
use serde::{Deserialize, Serialize};

const GATEWAY_URL: &str = "https://www.deezer.com/ajax/gw-light.php";
const MEDIA_URL_API: &str = "https://media.deezer.com/v1/get_url";

#[derive(Clone, Debug)]
pub struct Deezer {
    client: reqwest::Client,
    arl: String,
    session_id: String,
    license_token: String,
    api_token: String,
    user_id: i64,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub struct Response {
    results: Res,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Res {
    #[serde(rename(deserialize = "USER"))]
    user: User,
    #[serde(rename(deserialize = "SESSION_ID"))]
    session_id: String,
    #[serde(rename(deserialize = "checkForm"))]
    check_form: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    #[serde(rename(deserialize = "OPTIONS"))]
    options: Options,
    #[serde(rename(deserialize = "USER_ID"))]
    user_id: i64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Options {
    license_token: String,
}

impl Deezer {
    pub async fn new(arl: String) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(COOKIE, HeaderValue::from_str(&format!("arl={arl}"))?);
        headers.insert(
            USER_AGENT,
            HeaderValue::from_static(
                "Mozilla/5.0 (X11; Linux x86_64; rv:152.0) Gecko/20100101 Firefox/152.0",
            ),
        );

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .cookie_store(true)
            .build()?;

        let response: Response = client
            .post(format!(
                "{GATEWAY_URL}?method=deezer.getUserData&api_version=1.0&api_token="
            ))
            .header(reqwest::header::COOKIE, format!("arl={}", arl))
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body("{}")
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(Self {
            client,
            arl,
            session_id: response.results.session_id,
            license_token: response.results.user.options.license_token,
            api_token: response.results.check_form,
            user_id: response.results.user.user_id,
        })
    }
}

mod test {
    use crate::deezer;
    use anyhow::Result;
    use std::env;
    use tokio;

    #[tokio::test]
    async fn test_create_client() -> Result<()> {
        let arl = env::var("DEEZER_ARL")?;

        deezer::Deezer::new(arl).await?;
        Ok(())
    }
}
