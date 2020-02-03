use crate::components::todo_input::TodoInput;
use crate::components::todo_list::TodoList;
use crate::models::todo::{Todos};
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew::services::ConsoleService;

pub struct App {
    link: ComponentLink<Self>,
    todos: Todos,
}

#[derive(Debug)]
pub enum Msg {
    AddTodo(String),
    DoneChange(usize),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            link,
            todos: Todos::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        ConsoleService::new().log(format!("{:?}", self.todos).as_str());
        match msg {
            Msg::AddTodo(text) => {
                self.todos.push(text);
            },
            Msg::DoneChange(index) => {
                self.on_done_change(index);
                ConsoleService::new().log(format!("{:?}", self.todos).as_str());
            }
        }
        true
    }

    fn view(&self) -> Html {
        ConsoleService::new().log(format!("{:?}", self.todos).as_str());
        html! {
            <div>
                <TodoInput oncomplete=self.link.callback(|text| Msg::AddTodo(text)) />
                <TodoList list=&self.todos ondonechange=self.link.callback(|e| Msg::DoneChange(e)) />
            </div>
        }
    }
}

impl App {
    fn on_done_change(&mut self, index: usize) {
        ConsoleService::new().log(format!("{:?}", self.todos).as_str());
        self.todos.switch(index)
    }
}
