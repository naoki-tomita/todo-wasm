use crate::components::todo_input::TodoInput;
use crate::components::todo_list::TodoList;
use crate::models::todo::{Todo, Todos};
use serde::{Deserialize, Serialize};
use serde_json::json;
use yew::format::Json;
use yew::format::Nothing;
use yew::services::fetch::{FetchTask, Request, Response};
use yew::services::{FetchService};
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
    UpdatedTodos,
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
        self.load();
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdatedTodos => {
                self.load();
                false
            }
            Msg::AddTodo(text) => {
                self.register_data(text);
                true
            }
            Msg::DoneChange(id) => {
                self.update_data(id);
                true
            }
            Msg::FetchedTodo(todos) => {
                self.task = None;
                self.todos = todos;
                true
            }
            Msg::Noop => false,
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class="container" style="padding-top: 20px">
                <TodoInput oncomplete=self.link.callback(|text| Msg::AddTodo(text)) />
                <TodoList list=&self.todos ondonechange=self.link.callback(|e| Msg::DoneChange(e)) />
            </div>
        }
    }
}

impl App {
    fn load(&mut self) {
        let request = Request::get("/v1/todos")
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

    fn register_data(&mut self, text: String) {
        let json = &json!({ "text": text });
        let request = Request::post("/v1/todos")
            .body(Json(json))
            .expect("Failed to build request.");

        self.task = Some(
            FetchService::new().fetch(
                request,
                self.link
                    .callback(|_: Response<Result<String, failure::Error>>| Msg::UpdatedTodos),
            ),
        );
    }

    fn update_data(&mut self, id: usize) {
        match self.todos.find(id) {
            Some(it) => {
                let json = &json!({ "done": !it.done });
                let request = Request::put(format!("/v1/todos/{}", id))
                    .body(Json(json))
                    .expect("Failed to build request.");

                self.task = Some(
                    FetchService::new().fetch(
                        request,
                        self.link
                            .callback(|_: Response<Result<String, failure::Error>>| Msg::UpdatedTodos),
                    ),
                );
            },
            _ => return
        }
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
                    id: it.id,
                    text: it.text.clone(),
                    done: it.done,
                })
                .collect(),
        }
    }
}

#[derive(Deserialize)]
pub struct TodoJson {
    id: usize,
    text: String,
    done: bool,
}

#[derive(Serialize)]
pub struct TodoRequestJson {
    text: String,
}
