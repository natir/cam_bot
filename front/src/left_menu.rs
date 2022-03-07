//! Left Menu

/* std use */

/* crate use */
use yew::prelude::*;

/* project use */

pub struct LeftMenu {}

impl Component for LeftMenu {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
        <li>
        <ul><a href="commands">{"Commands"}</a></ul>
        <ul><a href="timers">{"Timers"}</a></ul>
        <ul><a href="twitch">{"Twitch"}</a></ul>
        </li>
        }
    }
}
