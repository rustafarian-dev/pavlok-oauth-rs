
const BASE_URL: &str = "https://app.pavlok.com/";

#[derive(Debug, Clone)]
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
    pub access_token: String,
}

type Error = &'static str;

impl Oauth {
    pub fn new(client_id:String, client_secret: String, redirect_uri: String) -> Oauth {
        Oauth {
            client_secret,
            client_id,
            redirect_uri,
            client: reqwest::Client::new(),
        }
    }

    pub fn pavlok_redirect_uri(&self) -> Result<url::Url, url::ParseError> {
        url::Url::parse_with_params(
            &format!("{}/{}", BASE_URL, "oauth/authorize"),
            &[
                ("client_id", self.client_id.as_str()),
                ("redirect_uri", self.redirect_uri.as_str()),
                ("response_type", "code"),
            ],
        )
    }

    pub async fn get_token(&self, code: &str) -> Result<TokenInfo, Error> {
        let token_request = GetTokenRequest {
            client_secret: self.client_secret.as_str(),
            client_id: self.client_id.as_str(),
            code,
            grant_type: "authorize_code",
            redirect_uri: self.redirect_uri.as_str(),
        };

        self.client
        .post(format!("{}/{}", BASE_URL, "oauth/token"))
        .json(&token_request)
        .send().await
        .map_err(|e| {
            println!("Error: {}", e);
            "sending error"
        })?
        .json::<TokenInfo>().await
        .map_err(|e| {
            println!("Error: {}", e);
            "json"
        })
    }
}
