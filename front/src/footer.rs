//! Footer

/* std use */

/* crate use */
use wasm_bindgen::prelude::*;
use yew::prelude::*;

/* project use */

pub struct Footer {}

impl Component for Footer {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
        {"Unless otherwise stated, the content of the sites is placed under free license."}
        }
    }
}
