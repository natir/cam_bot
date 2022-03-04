//! Manage twitch return uri

/* std use */

/* crate use */

/* project use */
use crate::db;

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![get, authorize]
}

#[rocket::get("/")]
pub async fn get(
    conn: crate::Dbconn,
) -> Result<rocket::serde::json::Json<db::twitch::Twitch>, rocket::response::status::NotFound<String>>
{
    Ok(rocket::serde::json::Json(
        db::twitch::Twitch::get(conn).await.map_err(|e| {
            rocket::response::status::NotFound(format!("Get twitch info failled error: {}", e))
        })?,
    ))
}

#[rocket::get("/authorize?<client_id>&<secret>&<code>")]
pub async fn authorize(
    client_id: String,
    secret: String,
    code: String,
    conn: crate::Dbconn,
) -> Result<rocket::serde::json::Json<usize>, rocket::response::status::NotFound<String>> {
    let value = crate::twitch::request_token(&client_id, &secret, &code)
        .await
        .map_err(|e| {
            rocket::response::status::NotFound(format!(
                "Request token with code {} error: {}",
                code.clone(),
                e
            ))
        })?;

    Ok(rocket::serde::json::Json(
        crate::db::twitch::Twitch::update(value, &conn)
            .await
            .map_err(|e| {
                rocket::response::status::NotFound(format!(
                    "Update twitch information in database failled error: {}",
                    e
                ))
            })?,
    ))
}
