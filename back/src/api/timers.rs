//! All commands stuff

/* std use */

/* crate use */

/* std use */
use crate::db;

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![index, get, delete, insert, update]
}

#[rocket::get("/", format = "application/json")]
pub async fn index(
    conn: crate::Dbconn,
) -> Result<
    rocket::serde::json::Json<Vec<db::timers::Timer>>,
    rocket::response::status::NotFound<String>,
> {
    let mut ret = Vec::new();
    for cmd in db::timers::Timer::all(&conn)
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
) -> Result<rocket::serde::json::Json<db::timers::Timer>, rocket::response::status::NotFound<String>>
{
    Ok(rocket::serde::json::Json(
        db::timers::Timer::get(id, &conn).await.map_err(|e| {
            rocket::response::status::NotFound(format!(
                "Timers with id {} not found error: {}",
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
        db::timers::Timer::delete(id, &conn).await.map_err(|e| {
            rocket::response::status::NotFound(format!(
                "Timers with id {} not found error: {}",
                id, e
            ))
        })?,
    ))
}

#[rocket::post("/", data = "<timer>")]
pub async fn insert(
    timer: rocket::serde::json::Json<db::timers::Timer>,
    conn: crate::Dbconn,
) -> Result<rocket::serde::json::Json<usize>, rocket::response::status::NotFound<String>> {
    let move_value = timer.clone();

    Ok(rocket::serde::json::Json(
        db::timers::Timer::insert(move_value, &conn)
            .await
            .map_err(|e| {
                rocket::response::status::NotFound(format!(
                    "Failled to create command {:?} error: {}",
                    *timer, e
                ))
            })?,
    ))
}

#[rocket::post("/update/<id>", data = "<timer>")]
pub async fn update(
    id: i32,
    timer: rocket::serde::json::Json<db::timers::Timer>,
    conn: crate::Dbconn,
) -> Result<rocket::serde::json::Json<usize>, rocket::response::status::NotFound<String>> {
    let move_value = timer.clone();

    Ok(rocket::serde::json::Json(
        db::timers::Timer::update(id, move_value, &conn)
            .await
            .map_err(|e| {
                rocket::response::status::NotFound(format!(
                    "Failled to update command {} with value {:?} error: {}",
                    id, *timer, e
                ))
            })?,
    ))
}
