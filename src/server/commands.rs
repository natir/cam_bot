//! All commands stuff

/* std use */

/* crate use */

/* std use */
use crate::db;

#[rocket::get("/")]
pub async fn index(conn: crate::Dbconn) -> String {
    let mut ret = String::new();
    for cmd in db::commands::Command::all(&conn).await.unwrap() {
        ret += &format!("{:?}\n", cmd);
    }

    ret
}

#[rocket::get("/<id>")]
pub async fn get(id: i32, conn: crate::Dbconn) -> String {
    let cmd = db::commands::Command::get(id, &conn).await.unwrap();

    format!("{:?}", cmd)
}

#[rocket::delete("/<id>")]
pub async fn delete(id: i32, conn: crate::Dbconn) -> String {
    db::commands::Command::delete(id, &conn).await.unwrap();

    format!("Command {} was delete", id)
}
