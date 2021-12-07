//! Twitch relate function

/* std use */

/* crate use */

/* project use */

pub async fn new_connexion(client_id: String) -> crate::db::twitch::Twitch {
    let params = [
	("client_id", client_id),
	("redirect_uri", "localhost".to_string()),
	("response_type", "code".to_string()),
	("scope", "bits:read channel:manage:broadcast channel:manage:extensions channel:manage:extensions channel:manage:polls channel:manage:polls channel:manage:predictions channel:manage:redemptions channel:manage:schedule channel:manage:videos channel:read:editors channel:read:goals channel:read:hype_train channel:read:polls channel:read:predictions channel:read:redemptions channel:read:stream_key clips:edite moderation:read moderator:manage:banned_users moderator:read:blocked_terms moderator:manage:automod moderator:read:automod_settings moderator:manage:automod_settings moderator:read:chat_settings moderator:manage:chat_settings user:edit user:edit:follows user:manage:blocked_users user:read:blocked_users user:read:broadcast user:read:email user:read:follows user:read:subscriptions".to_string())
    ];
    let client = reqwest::Client::new();
    let res = client
        .get("https://id.twitch.tv/oauth2/authorize")
        .query(&params)
        .send()
        .await
        .unwrap();

    crate::db::twitch::Twitch {
        id: 0,
        token: "".to_string(),
        refresh_token: "".to_string(),
        expire_in: 0,
        generation_date: chrono::Utc::now().naive_utc().date(),
    }
}
