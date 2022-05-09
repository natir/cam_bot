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
            html! {
                   <twitch::Twitch>

                    <twitch::scope::Scope name="analytics:read:extensions"/>
            <twitch::scope::Scope name="analytics:read:games"/>
            <twitch::scope::Scope name="bits:read"/>
            <twitch::scope::Scope name="channel:edit:commercial"/>
            <twitch::scope::Scope name="channel:manage:broadcast"/>
            <twitch::scope::Scope name="channel:manage:extensions"/>
            <twitch::scope::Scope name="channel:manage:polls"/>
            <twitch::scope::Scope name="channel:manage:predictions"/>
            <twitch::scope::Scope name="channel:manage:redemptions"/>
            <twitch::scope::Scope name="channel:manage:schedule"/>
            <twitch::scope::Scope name="channel:manage:videos"/>
            <twitch::scope::Scope name="channel:read:editors"/>
            <twitch::scope::Scope name="channel:read:goals"/>
            <twitch::scope::Scope name="channel:read:hype_train"/>
            <twitch::scope::Scope name="channel:read:polls"/>
            <twitch::scope::Scope name="channel:read:predictions"/>
            <twitch::scope::Scope name="channel:read:redemptions"/>
            <twitch::scope::Scope name="channel:read:stream_key"/>
            <twitch::scope::Scope name="channel:read:subscriptions"/>
            <twitch::scope::Scope name="clips:edit"/>
            <twitch::scope::Scope name="moderation:read"/>
            <twitch::scope::Scope name="moderator:manage:banned_users"/>
            <twitch::scope::Scope name="moderator:read:blocked_terms"/>
            <twitch::scope::Scope name="moderator:manage:blocked_terms"/>
            <twitch::scope::Scope name="moderator:manage:automod"/>
            <twitch::scope::Scope name="moderator:read:automod_settings"/>
            <twitch::scope::Scope name="moderator:manage:automod_settings"/>
            <twitch::scope::Scope name="moderator:read:chat_settings"/>
            <twitch::scope::Scope name="moderator:manage:chat_settings"/>
            <twitch::scope::Scope name="user:edit"/>
            <twitch::scope::Scope name="user:edit:follows"/>
            <twitch::scope::Scope name="user:manage:blocked_users"/>
            <twitch::scope::Scope name="user:read:blocked_users"/>
            <twitch::scope::Scope name="user:read:broadcast"/>
            <twitch::scope::Scope name="user:read:email"/>
            <twitch::scope::Scope name="user:read:follows"/>
            <twitch::scope::Scope name="user:read:subscriptions"/>

               </twitch::Twitch>
               }
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
