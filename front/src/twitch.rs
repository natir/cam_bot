//! Header

/* std use */

use std::fmt::Display;

/* crate use */
use yew::prelude::*;

/* project use */

fn fetch_twitch_data() -> String {
    //let res = reqwest::blocking::get("/api/twitch").unwrap();

    "Twitch information".to_string()
}

pub enum Message {
    SelectAllScope,
}

#[derive(Debug, PartialEq, Clone, Properties)]
pub struct Scope {
    name: String,
    pub toggle: bool,
}

impl Display for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "name: {} toggle: {}", self.name, self.toggle)
    }
}

impl Component for Scope {
    type Message = Message;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            name: "".to_string(),
            toggle: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::SelectAllScope => self.toggle = true,
        }

        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <label> <input type={ "checkbox" } name={ self.name.clone() } checked={ self.toggle } /> { self.name.clone() } </label>
        }
    }
}

fn avaible_scopes() -> Vec<Scope> {
    vec![
        Scope {
            name: "analytics:read:extensions".to_string(),
            toggle: false,
        },
        Scope {
            name: "analytics:read:games".to_string(),
            toggle: false,
        },
        Scope {
            name: "bits:read".to_string(),
            toggle: false,
        },
        Scope {
            name: "channel:edit:commercial".to_string(),
            toggle: false,
        },
        Scope {
            name: "channel:manage:broadcast".to_string(),
            toggle: false,
        },
        Scope {
            name: "channel:manage:extensions".to_string(),
            toggle: false,
        },
        Scope {
            name: "channel:manage:polls".to_string(),
            toggle: false,
        },
        Scope {
            name: "channel:manage:predictions".to_string(),
            toggle: false,
        },
        Scope {
            name: "channel:manage:redemptions".to_string(),
            toggle: false,
        },
        Scope {
            name: "channel:manage:schedule".to_string(),
            toggle: false,
        },
        Scope {
            name: "channel:manage:videos".to_string(),
            toggle: false,
        },
        Scope {
            name: "channel:read:editors".to_string(),
            toggle: false,
        },
        Scope {
            name: "channel:read:goals".to_string(),
            toggle: false,
        },
        Scope {
            name: "channel:read:hype_train".to_string(),
            toggle: false,
        },
        Scope {
            name: "channel:read:polls".to_string(),
            toggle: false,
        },
        Scope {
            name: "channel:read:predictions".to_string(),
            toggle: false,
        },
        Scope {
            name: "channel:read:redemptions".to_string(),
            toggle: false,
        },
        Scope {
            name: "channel:read:stream_key".to_string(),
            toggle: false,
        },
        Scope {
            name: "channel:read:subscriptions".to_string(),
            toggle: false,
        },
        Scope {
            name: "clips:edit".to_string(),
            toggle: false,
        },
        Scope {
            name: "moderation:read".to_string(),
            toggle: false,
        },
        Scope {
            name: "moderator:manage:banned_users".to_string(),
            toggle: false,
        },
        Scope {
            name: "moderator:read:blocked_terms".to_string(),
            toggle: false,
        },
        Scope {
            name: "moderator:manage:blocked_terms".to_string(),
            toggle: false,
        },
        Scope {
            name: "moderator:manage:automod".to_string(),
            toggle: false,
        },
        Scope {
            name: "moderator:read:automod_settings".to_string(),
            toggle: false,
        },
        Scope {
            name: "moderator:manage:automod_settings".to_string(),
            toggle: false,
        },
        Scope {
            name: "moderator:read:chat_settings".to_string(),
            toggle: false,
        },
        Scope {
            name: "moderator:manage:chat_settings".to_string(),
            toggle: false,
        },
        Scope {
            name: "user:edit".to_string(),
            toggle: false,
        },
        Scope {
            name: "user:edit:follows".to_string(),
            toggle: false,
        },
        Scope {
            name: "user:manage:blocked_users".to_string(),
            toggle: false,
        },
        Scope {
            name: "user:read:blocked_users".to_string(),
            toggle: false,
        },
        Scope {
            name: "user:read:broadcast".to_string(),
            toggle: false,
        },
        Scope {
            name: "user:read:email".to_string(),
            toggle: false,
        },
        Scope {
            name: "user:read:follows".to_string(),
            toggle: false,
        },
        Scope {
            name: "user:read:subscriptions".to_string(),
            toggle: false,
        },
    ]
}

#[derive(Properties, PartialEq)]
pub struct ListProps {
    #[prop_or(avaible_scopes())]
    pub children: Vec<Scope>,
}

pub struct Twitch {}

impl Component for Twitch {
    type Message = Message;
    type Properties = ListProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let check_all = ctx
            .link()
            .callback(|_e: yew::MouseEvent| Message::SelectAllScope);
        let text = fetch_twitch_data();

        html! {
             <>
                <h2>{ "Twitch" }</h2>
                <p>{ text }</p>
                <section id="form">
        <form>
                <li id="scopes">
        { ctx.props().children[0].clone() }
        </li>
        <button onclick={check_all}>{ "Select all" }</button>
        <input type="text" name="client_id" /><label>{ "Client Id" }</label>
        </form>
                </section>
                </>
         }
    }
}
