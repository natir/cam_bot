//! Twitch relate function

/* std use */

/* crate use */

/* project use */
use crate::error;

pub async fn request_authorize(client_id: &str) -> error::Result<()> {
    let params = [
	("client_id", client_id),
	("redirect_uri", "https://localhost:8080/api/twitch/authorize"),
	("response_type", "code"),
	("scope", "bits:read channel:manage:broadcast channel:manage:extensions channel:manage:extensions channel:manage:polls channel:manage:polls channel:manage:predictions channel:manage:redemptions channel:manage:schedule channel:manage:videos channel:read:editors channel:read:goals channel:read:hype_train channel:read:polls channel:read:predictions channel:read:redemptions channel:read:stream_key clips:edit moderation:read moderator:manage:banned_users moderator:read:blocked_terms moderator:manage:automod moderator:read:automod_settings moderator:manage:automod_settings moderator:read:chat_settings moderator:manage:chat_settings user:edit user:edit:follows user:manage:blocked_users user:read:blocked_users user:read:broadcast user:read:email user:read:follows user:read:subscriptions")
    ];

    let client = reqwest::Client::builder().connect_timeout(std::time::Duration::from_secs(5)).connection_verbose(true).build().unwrap();

    let res = client
        .get("https://id.twitch.tv/oauth2/authorize")
        .query(&params)
	.send().await
        .map_err(|error| error::Error::Reqwest(error::Reqwest::Twitch { error }))?;

    match res.status() {
        reqwest::StatusCode::OK => Ok(()),
        code => Err(error::Error::Reqwest(error::Reqwest::Authorize { code })),
    }
}

#[derive(rocket::serde::Deserialize)]
#[serde(crate = "rocket::serde")]
struct TwitchToken {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i32,
    pub scope: Vec<String>,
    pub token_type: String,
}

impl std::convert::From<TwitchToken> for crate::db::twitch::Twitch {
    fn from(ori: TwitchToken) -> Self {
        crate::db::twitch::Twitch {
            id: 1,
            token: ori.access_token,
            refresh_token: ori.refresh_token,
            expire_in: ori.expires_in,
            generation_date: chrono::Utc::now().naive_utc().date(),
        }
    }
}

pub async fn request_token(
    client_id: &str,
    secret: &str,
    code: &str,
) -> error::Result<crate::db::twitch::Twitch> {
    let params = [
        ("client_id", client_id),
        ("client_secret", secret),
        ("code", code),
        ("grant_type", "authorization_code"),
        ("redirect_uri", "localhost/api/twitch/token"),
    ];

    let client = reqwest::Client::new();

    let res = client
        .post("https://id.twitch.tv/oauth2/token")
        .query(&params)
        .send()
        .await
        .map_err(|error| error::Error::Reqwest(error::Reqwest::Twitch { error }))?;

    if res.status() != reqwest::StatusCode::OK {
        return Err(error::Error::Reqwest(error::Reqwest::Authorize {
            code: res.status(),
        }));
    }

    let token = res
        .json::<TwitchToken>()
        .await
        .map_err(|error| error::Error::Reqwest(error::Reqwest::DeserilizeToken { error }))?;

    Ok(crate::db::twitch::Twitch::from(token))
}
