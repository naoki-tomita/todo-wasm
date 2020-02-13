use crate::domains::todo::{Stat, Todo, Todos};
use crate::drivers::todo_driver;
use crate::drivers::todo_driver::TodoEntity;
use crate::error::Error;
use crate::ports::todo_port::TodoPort;

#[derive(Clone)]
pub struct TodoGateway {}

impl TodoPort for TodoGateway {
    fn get_list(&self) -> Result<Todos, Error> {
        Ok(Todos::from(todo_driver::get_todos()?))
    }
    fn register(&self, todo: Todo) -> std::result::Result<Todo, Error> {
        Ok(Todo::from(todo_driver::register(
            todo.text.0,
            todo.done == Stat::Done,
        )?))
    }
}

impl Todos {
    fn from(values: Vec<TodoEntity>) -> Self {
        Todos::new(values.iter().map(|it| Todo::from(it.clone())).collect())
    }
}

impl Todo {
    fn from(value: TodoEntity) -> Self {
        Todo::new(value.text, value.done)
    }
}
