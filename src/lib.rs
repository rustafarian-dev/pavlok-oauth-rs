use reqwest;
use serde;

const BASE_URL: &'static str = "https://app.pavlok.com/";

#[derive(Debug)]
pub struct Oauth {
    client: reqwest::Client,
    client_id: String,
    client_secret: String,
    redirect_uri: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct GetTokenRequest<'a> {
    client_id: &'a str,
    client_secret: &'a str,
    redirect_uri: &'a str,
    code: &'a str,
    grant_type: &'a str,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TokenInfo {
    access_token: String,
}

impl Oauth {
    pub fn new(client_id:String, client_secret: String, redirect_uri: String) -> Oauth {
        return Oauth {
            client_secret: client_secret,
            client_id: client_id,
            client: reqwest::Client::new(),
            redirect_uri: redirect_uri,
        }
    }

    pub fn redirect_uri(&self) -> Result<url::Url, url::ParseError> {
        url::Url::parse_with_params(
            &format!("{}/{}", BASE_URL, "oauth/authorize"),
            &[
                ("client_id", self.client_id.as_str()),
                ("redirect_uri", self.redirect_uri.as_str()),
                ("response_type", "code"),
            ],
        )
    }

    pub async fn get_token(&self, code: &str) -> Result<TokenInfo, &'static str> {
        let tokenRequest = GetTokenRequest {
            client_secret: self.client_secret.as_str(),
            client_id: self.client_id.as_str(),
            code: code,
            grant_type: "authorize_code",
            redirect_uri: self.redirect_uri.as_str(),
        };

        Ok(
            self.client
            .post(format!("{}/{}", BASE_URL, "oauth/token"))
            .json(&tokenRequest)
            .send().await
            .map_err(|err| "Bad")?
            .json::<TokenInfo>().await
            .map_err(|err| "Bad")?
        )
    }
}
