//! Header

/* std use */

/* crate use */
use yew::prelude::*;

/* project use */

/* mod declaration */
pub mod scope;

fn fetch_twitch_data() -> String {
    //let res = reqwest::blocking::get("/api/twitch").unwrap();

    "Twitch information".to_string()
}

pub enum Message {
    SelectAllScope,
}

#[derive(Properties, PartialEq)]
pub struct ListProps {
    #[prop_or_default]
    pub children: ChildrenWithProps<scope::Scope>,
}

pub struct Twitch {}

impl Component for Twitch {
    type Message = Message;
    type Properties = ListProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        log::debug!("Twitch update");
        match msg {
            self::Message::SelectAllScope => {
                log::debug!("SelectAllScope {}", ctx.props().children.len());
                ctx.props().children.iter().for_each(|mut item| {
                    let mut props = std::rc::Rc::make_mut(&mut item.props);
                    props.toggle = true;
                    log::debug!("{} {}", props.name, props.toggle);
                });

                true
            }
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        log::debug!("rendered");
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let onclick = link.callback(|_| {
            log::debug!("BUTTON CLICK");

            Message::SelectAllScope
        });
        let text = fetch_twitch_data();

        html! {
            <>
                <h2>{ "Twitch" }</h2>
                <p>{ text }</p>
                <section id="form">
        <form id="twitch_form">
        <li>
        { for ctx.props().children.iter() }
            </li>
        <button type="button" onclick={onclick}>{ "Select all" }</button>
        <input type="text" name="client_id" /><label>{ "Client Id" }</label>
        </form>

        <button type="submit" form="twitch_form">{ "Submit" }</button>

                </section>
                </>
        }
    }
}
