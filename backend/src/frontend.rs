//! Static routes

/* std use */

/* crate use */

/* std use */

#[rocket::get("/")]
pub async fn file() -> rocket_dyn_templates::Template {
    let context: std::collections::HashMap<&str, &str> = std::collections::HashMap::new();

    rocket_dyn_templates::Template::render("index", &context)
}
