use crate::domains::todo::{Description, Stat, Todo};
use crate::gateways::todo_gateway::TodoGateway;
use crate::usecases::get_todo_list;
use crate::usecases::register_todo;
use rocket::response::status::Created;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TodosJson {
    values: Vec<TodoJson>,
}

#[derive(Serialize, Deserialize)]
pub struct TodoJson {
    id: usize,
    text: String,
    done: bool,
}

#[derive(Serialize, Deserialize)]
pub struct TodoInputJson {
    text: String
}

lazy_static! {
    static ref TODO_PORT: TodoGateway = { TodoGateway {} };
}

#[get("/todos")]
pub fn get() -> Json<TodosJson> {
    let port = TODO_PORT.clone();
    let result = get_todo_list::execute(port);
    match result {
        Ok(todos) => Json(TodosJson {
            values: todos
                .values
                .iter()
                .map(|todo| TodoJson {
                    id: todo.id.0,
                    text: todo.text.0.clone(),
                    done: todo.done == Stat::Done,
                })
                .collect(),
        }),
        Err(e) => panic!(format!("{:?}", e)),
    }
}

#[post("/todos", data = "<todo>")]
pub fn post(todo: Json<TodoInputJson>) -> Created<Json<TodoJson>> {
    let port = TODO_PORT.clone();
    let result = register_todo::execute(port, Description(todo.text.clone()));
    match result {
        Ok(todo) => Created(
            "path".to_string(),
            Some(Json(TodoJson {
                id: todo.id.0,
                text: todo.text.0.clone(),
                done: todo.done == Stat::Done,
            })),
        ),
        Err(e) => panic!(format!("{:?}", e)),
    }
}

#[put("/todos/<id>")]
pub fn put(id: u32) -> Json<()> {
    unimplemented!()
}
