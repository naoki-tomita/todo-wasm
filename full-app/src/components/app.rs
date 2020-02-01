use yew::services::ConsoleService;
use crate::components::todo_input::TodoInput;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct App {
    link: ComponentLink<Self>,
    console: ConsoleService,
}

#[derive(Debug)]
pub enum Msg {
    AddTodo(String),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App { link, console: ConsoleService::new() }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        self.console.log(format!("{:?}", msg).as_str());
        true
    }

    fn view(&self) -> Html {
        html! {
            <TodoInput oncomplete=self.link.callback(|_| Msg::AddTodo("hello world".to_string())) />
        }
    }
}
