//! Frontend of cam_bot

/* std use */

/* crate use */
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

/* project use */

/* mod declaration */
mod footer;
mod header;
mod left_menu;
mod twitch;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/commands")]
    Commands,
    #[at("/timers")]
    Timers,
    #[at("/twitch")]
    Twitch,
    #[not_found]
    #[at("/404")]
    NotFound,
}

struct Skeleton {}

impl Component for Skeleton {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <header>
                <header::Header />
                </header>
                <nav>
                <left_menu::LeftMenu />
        </nav>
        <main>
        <BrowserRouter>
        <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
        </main>
        <footer>
        <footer::Footer />
        </footer>
                </>
        }
    }
}

fn switch(routes: &Route) -> Html {
    match routes.clone() {
        Route::Home => {
            html! { "Home" }
        }
        Route::Commands => {
            html! { "Commands"}
        }
        Route::Timers => {
            html! { "Timers" }
        }
        Route::Twitch => {
            html! { <twitch::Twitch /> }
        }
        Route::NotFound => {
            html! { "404" }
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::start_app::<Skeleton>();

    Ok(())
}
