use crate::domains::todo::Id;
use crate::domains::todo::Stat;
use crate::domains::todo::{Description, IdentifiedTodo, IdentifiedTodos, Todo, Todos};
use crate::drivers::todo_driver;
use crate::drivers::todo_driver::TodoEntity;
use crate::error::Error;
use crate::ports::todo_port::TodoPort;

#[derive(Clone)]
pub struct TodoGateway {}

impl TodoPort for TodoGateway {
    fn get_list(&self) -> Result<IdentifiedTodos, Error> {
        Ok(IdentifiedTodos::from(todo_driver::get_todos()?))
    }
    fn register(&self, text: Description) -> Result<IdentifiedTodo, Error> {
        Ok(IdentifiedTodo::from(todo_driver::register(text.0, false)?))
    }
    fn update(
        &self,
        id: Id,
        text: Option<Description>,
        done: Option<Stat>,
    ) -> Result<IdentifiedTodo, Error> {
        Ok(IdentifiedTodo::from(todo_driver::update(
            id.0,
            text.map_or(None, |t| Some(t.0)),
            done.map_or(None, |d| Some(d == Stat::Done)),
        )?))
    }
}

impl IdentifiedTodos {
    fn from(values: Vec<TodoEntity>) -> Self {
        IdentifiedTodos::new(
            values
                .iter()
                .map(|it| IdentifiedTodo::from(it.clone()))
                .collect(),
        )
    }
}

impl IdentifiedTodo {
    fn from(value: TodoEntity) -> Self {
        IdentifiedTodo::new(value.id, value.text, value.done)
    }
}
