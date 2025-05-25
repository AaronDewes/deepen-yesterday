//use base64::Engine;
//use base64::prelude::BASE64_STANDARD_NO_PAD;
use deepen_api::models::PostAuthRefreshRequest;

pub async fn get_token(
    configuration: &deepen_api::apis::configuration::Configuration,
    locale: &str,
) -> anyhow::Result<String> {
    let token = deepen_api::apis::auth_api::auth_refresh(
        configuration,
        locale,
        Some("android"),
        Some("1.0.4"),
        None,
        None,
        Some(PostAuthRefreshRequest {
            refresh_token: "INSERT_TOKEN_HERE".to_string(),
            device_account_id: "INSERT_TOKEN_HERE".to_string(),
        }),
    ).await?;
    Ok(token.access_token)
}

pub struct Client {
    config: deepen_api::apis::configuration::Configuration,
    //expiry: Option<chrono::DateTime<chrono::Utc>>,
}

impl Client {
    pub fn new(base_uri: String) -> Self {
        Self {
            config: deepen_api::apis::configuration::Configuration {
                base_path: base_uri,
                user_agent: Some("okhttp/4.12.0".to_string()),
                client: reqwest::Client::builder()
                    .user_agent("okhttp/4.12.0")
                    .build()
                    .expect("Failed to create reqwest client"),
                basic_auth: None,
                oauth_access_token: None,
                bearer_access_token: None,
                api_key: None,
            },
            //expiry: None,
        }
    }
    pub async fn get_config(
        &mut self,
        locale: &str,
    ) -> &deepen_api::apis::configuration::Configuration {
        // TODO: We don't need to refresh the token on every request, but that would require better error handling - We can't fully rely on expiry yet
        let token = get_token(&self.config, locale).await.unwrap();
        /*{
            let jwt_main_part = token.split('.').nth(1).unwrap();
            let decoded = BASE64_STANDARD_NO_PAD
                .decode(jwt_main_part)
                .expect("Failed to decode JWT");
            let payload: serde_json::Value =
                serde_json::from_slice(&decoded).expect("Failed to parse JWT payload");
            let exp = payload["exp"]
                .as_i64()
                .expect("Failed to get exp from JWT payload");
            let exp = chrono::NaiveDateTime::from_timestamp(exp, 0);
            self.expiry = Some(chrono::DateTime::<chrono::Utc>::from_utc(exp, chrono::Utc));
            tracing::debug!("Token expires at {}", exp);
        }*/
        self.config.bearer_access_token = Some(token);
        &self.config
    }
}
