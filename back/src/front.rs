//! Static routes

/* std use */

/* crate use */

/* std use */

#[rocket::get("/<_..>", rank = 3)]
pub async fn front() -> Option<rocket::fs::NamedFile> {
    rocket::fs::NamedFile::open("templates/index.html")
        .await
        .ok()
}
