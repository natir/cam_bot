//! Static routes

/* std use */

/* crate use */

/* std use */

use std::path::PathBuf;

#[rocket::get("/<path..>?query")]
pub async fn file(path: PathBuf) -> rocket_dyn_templates::Template {
    let context: std::collections::HashMap<&str, &str> = std::collections::HashMap::new();

    rocket_dyn_templates::Template::render("index", &context)
}
