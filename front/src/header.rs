//! Header

/* std use */

/* crate use */
use wasm_bindgen::prelude::*;
use yew::prelude::*;

/* project use */

pub struct Header {}

impl Component for Header {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
        <h1><a href="/">{ "Cam_Bot" }</a></h1>
        }
    }
}
