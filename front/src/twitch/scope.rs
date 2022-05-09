//! twitch Scope

/* std use */

/* crate use */
use yew::prelude::*;

/* project use */

#[derive(Properties, Clone, PartialEq)]
pub struct Properties {
    pub name: String,
    #[prop_or_default]
    pub toggle: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Scope {
    name: String,
    toggle: bool,
}

impl Component for Scope {
    type Message = crate::twitch::Message;
    type Properties = Properties;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            name: ctx.props().name.clone(),
            toggle: ctx.props().toggle,
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        log::debug!("rendered");

        if !first_render {
            self.update(_ctx, Self::Message::SelectAllScope);
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, message: Self::Message) -> bool {
        log::debug!("update");

        match message {
            Self::Message::SelectAllScope => true,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        log::debug!("{} {}", ctx.props().name, ctx.props().toggle);

        html! {
            <ul>
        <label>

        if ctx.props().toggle {
        <input type={ "checkbox" } name={ ctx.props().name.clone() } checked=true /> { ctx.props().name.clone() }
        } else {
        <input type={ "checkbox" } name={ ctx.props().name.clone() } /> { ctx.props().name.clone() }
        }

        </label>
        </ul>
        }
    }
}
