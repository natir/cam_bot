//! Static routes

/* std use */

/* crate use */

/* std use */

#[rocket::get("/<path..>", rank=1)]
pub async fn file(path: std::path::PathBuf) -> std::io::Result<rocket::fs::NamedFile> {
    let mut path = std::path::Path::new("frontend").join(path);
    if path.is_dir() {
        path.push("index.html");
    }

    println!("{:?}", path);
    rocket::fs::NamedFile::open(path).await
}
