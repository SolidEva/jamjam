use anyhow::{anyhow, Context, Result};
use log::{debug, error, info, log_enabled, Level};
use reqwest::header::{HeaderMap, HeaderValue, COOKIE, USER_AGENT};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

const API_URL: &str = "https://pipe.deezer.com/api";
const AUTH_URL: &str = "https://auth.deezer.com/login/arl";

#[derive(Clone, Debug)]
pub struct Deezer {
    client: reqwest::Client,
    arl: String,
}

impl Deezer {
    async fn login(client: &reqwest::Client, arl: &String) -> Result<()> {
        // request jwt token in (p)ayload aka body, request refresh token in cookie, provide (i)dentity aka arl in cookie
        let magic_params = [("jo", "p"), ("rto", "c"), ("i", "c")];

        client
            .post(AUTH_URL)
            .query(&magic_params)
            .header(reqwest::header::COOKIE, format!("arl={}", arl))
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    async fn refresh_login(self: &mut Self) -> Result<&mut Self> {
        Ok(self)
    }

    pub async fn new(arl: String) -> Result<Self> {
        let client = reqwest::Client::builder()
            .cookie_store(true)
            .connection_verbose(true)
            .build()?;
        Deezer::login(&client, &arl).await?;

        Ok(Self { client, arl })
    }

    // async fn

    // async fn search(&self, query: &String) -> Result<()> {
    //     Ok(())
    // }
}

#[cfg(test)]
mod test {
    use crate::deezer::Deezer;
    use anyhow::Result;
    use std::env;
    use tokio;

    #[cfg(test)]
    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[cfg(test)]
    async fn create_test_client() -> Result<Deezer> {
        init();
        let arl = env::var("DEEZER_ARL")?;

        Deezer::new(arl).await
    }

    #[tokio::test]
    async fn test_create_client() -> Result<()> {
        create_test_client().await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_refresh_login() -> Result<()> {
        let mut deezer = create_test_client().await?;
        deezer.refresh_login().await?;
        Ok(())
    }

    // #[tokio::test]
    // async fn test_search() -> Result<()> {
    //     let arl = env::var("DEEZER_ARL")?;

    //     let api = deezer::Deezer::new(arl).await?;
    //     api.search(&"moon tattoo".to_string()).await?;
    //     Ok(())
    // }
}
