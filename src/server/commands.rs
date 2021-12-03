//! All commands stuff

/* std use */

/* crate use */

/* std use */
use crate::db;

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![index, get, delete, insert]
}

#[rocket::get("/", format = "application/json")]
pub async fn index(
    conn: crate::Dbconn,
) -> Result<
    rocket::serde::json::Json<Vec<db::commands::Command>>,
    rocket::response::status::NotFound<String>,
> {
    let mut ret = Vec::new();
    for cmd in db::commands::Command::all(&conn)
        .await
        .map_err(|e| rocket::response::status::NotFound(format!("Not found error: {}", e)))?
    {
        ret.push(cmd);
    }

    Ok(rocket::serde::json::Json(ret))
}

#[rocket::get("/<id>", format = "application/json")]
pub async fn get(
    id: i32,
    conn: crate::Dbconn,
) -> Result<
    rocket::serde::json::Json<db::commands::Command>,
    rocket::response::status::NotFound<String>,
> {
    Ok(rocket::serde::json::Json(
        db::commands::Command::get(id, &conn).await.map_err(|e| {
            rocket::response::status::NotFound(format!(
                "Commands with id {} not found error: {}",
                id, e
            ))
        })?,
    ))
}

#[rocket::delete("/<id>", format = "application/json")]
pub async fn delete(
    id: i32,
    conn: crate::Dbconn,
) -> Result<rocket::serde::json::Json<usize>, rocket::response::status::NotFound<String>> {
    Ok(rocket::serde::json::Json(
        db::commands::Command::delete(id, &conn)
            .await
            .map_err(|e| {
                rocket::response::status::NotFound(format!(
                    "Commands with id {} not found error: {}",
                    id, e
                ))
            })?,
    ))
}

#[rocket::post("/", data = "<command>")]
pub async fn insert(
    command: rocket::serde::json::Json<db::commands::Command>,
    conn: crate::Dbconn,
) -> Result<rocket::serde::json::Json<usize>, rocket::response::status::NotFound<String>> {
    let move_value = command.clone();

    Ok(rocket::serde::json::Json(
        db::commands::Command::insert(move_value, &conn)
            .await
            .map_err(|e| {
                rocket::response::status::NotFound(format!(
                    "Failled to create command {:?} error: {}",
                    *command, e
                ))
            })?,
    ))
}
