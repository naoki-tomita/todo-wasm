use crate::domains::todo::{Description, Id, Stat};
use crate::gateways::todo_gateway::TodoGateway;
use crate::usecases::{get_todo_list, register_todo, update_todo};
use rocket::response::status::{Created, NotFound};
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
pub struct TodoRegisterJson {
    text: String,
}

#[derive(Serialize, Deserialize)]
pub struct TodoUpdateJson {
    text: Option<String>,
    done: Option<bool>,
}

lazy_static! {
    static ref TODO_PORT: TodoGateway = TodoGateway {};
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
                .map(|data| TodoJson {
                    id: data.id.0,
                    text: data.todo.text.0.clone(),
                    done: data.todo.done == Stat::Done,
                })
                .collect(),
        }),
        Err(e) => panic!(format!("{:?}", e)),
    }
}

#[post("/todos", data = "<todo>")]
pub fn post(todo: Json<TodoRegisterJson>) -> Created<Json<TodoJson>> {
    let port = TODO_PORT.clone();
    let result = register_todo::execute(port, Description(todo.text.clone()));
    match result {
        Ok(data) => Created(
            "path".to_string(),
            Some(Json(TodoJson {
                id: data.id.0,
                text: data.todo.text.0.clone(),
                done: data.todo.done == Stat::Done,
            })),
        ),
        Err(e) => panic!(format!("{:?}", e)),
    }
}

#[put("/todos/<id>", data = "<todo>")]
pub fn put(id: usize, todo: Json<TodoUpdateJson>) -> Result<Json<TodoJson>, NotFound<Json<HashMap<String, String>>>> {
    let port = TODO_PORT.clone();
    let result = update_todo::execute(
        port,
        Id(id),
        todo.text
            .as_ref()
            .map_or(None, |t| Some(Description(t.clone()))),
        todo.done
            .map_or(None, |d| Some(if d { Stat::Done } else { Stat::UnDone })),
    );
    match result {
        Ok(data) => Ok(Json(TodoJson {
            id: data.id.0,
            text: data.todo.text.0.clone(),
            done: data.todo.done == Stat::Done,
        })),
        Err(e) => {
            let mut err: HashMap<String, String> = HashMap::new();
            err.insert("error".to_string(), format!("{:?}", e));
            Err(NotFound(Json(err)))
        },
    }
}
