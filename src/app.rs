use yew::prelude::*;
use super::todo::TodoApp;
use super::constants::TITLE;
// use super::todo::TodoApp;

pub struct App {}

pub enum Msg {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }
    fn view(&self) -> Html {
        html! {
            <div>
                <p>{ TITLE }</p>
                <p><TodoApp /></p>
            </div>
        }
    }
}
