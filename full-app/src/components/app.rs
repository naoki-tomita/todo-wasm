use crate::components::todo_input::TodoInput;
use crate::components::todo_list::TodoList;
use crate::models::todo::{Todo, Todos};
use serde::Deserialize;
use yew::format::Json;
use yew::format::Nothing;
use yew::services::fetch::{FetchTask, Request, Response};
use yew::services::{ConsoleService, FetchService};
use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct App {
    link: ComponentLink<Self>,
    todos: Todos,
    task: Option<FetchTask>,
}

#[derive(Debug)]
pub enum Msg {
    AddTodo(String),
    DoneChange(usize),
    FetchedTodo(Todos),
    Noop,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            link,
            todos: Todos::new(),
            task: None,
        }
    }

    fn mounted(&mut self) -> bool {
        self.update_data();
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        ConsoleService::new().log(format!("{:?}", self.todos).as_str());
        match msg {
            Msg::AddTodo(text) => {
                self.todos.push(text);
                true
            }
            Msg::DoneChange(index) => {
                self.on_done_change(index);
                ConsoleService::new().log(format!("{:?}", self.todos).as_str());
                true
            }
            Msg::FetchedTodo(todos) => {
                self.task = None;
                ConsoleService::new().log(format!("{:?}", todos).as_str());
                self.todos = todos;
                true
            }
            Msg::Noop => false,
        }
    }

    fn view(&self) -> Html {
        ConsoleService::new().log(format!("{:?}", self.todos).as_str());
        html! {
            <div class="container">
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

    fn update_data(&mut self) {
        let request = Request::get("http://localhost/v1/todos")
            .body(Nothing)
            .expect("Failed to build request.");

        self.task = Some(FetchService::new().fetch(
            request,
            self.link.callback(
                |response: Response<Json<Result<TodosJson, failure::Error>>>| {
                    if let (meta, Json(Ok(body))) = response.into_parts() {
                        if meta.status.is_success() {
                            return Msg::FetchedTodo(body.to_model());
                        }
                    }
                    Msg::Noop
                },
            ),
        ));
    }
}

#[derive(Deserialize)]
pub struct TodosJson {
    values: Vec<TodoJson>,
}

impl TodosJson {
    fn to_model(&self) -> Todos {
        Todos {
            list: self
                .values
                .iter()
                .map(|it| Todo {
                    text: it.text.clone(),
                    done: it.done,
                })
                .collect(),
        }
    }
}

#[derive(Deserialize)]
pub struct TodoJson {
    text: String,
    done: bool,
}
