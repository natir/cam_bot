//! Header

/* std use */

/* crate use */
use yew::prelude::*;

use wasm_bindgen::JsCast as _;

/* project use */

fn fetch_twitch_data() -> String {
    let res = reqwest::blocking::get("/api/twitch").unwrap();

    "Caca prout".to_string()
}

pub struct Twitch {}

impl Component for Twitch {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let text = fetch_twitch_data();

        html! {
            <>
                <h2>{ "Twitch" }</h2>
                <p>{ text }</p>
        </>
        }
    }
}
