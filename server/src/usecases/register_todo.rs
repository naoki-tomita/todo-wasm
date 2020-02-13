use crate::domains::todo::{Todo, Todos};
use crate::error::Error;
use crate::ports::todo_port::TodoPort;

pub fn execute(todo_port: impl TodoPort, todo: Todo) -> Result<Todo, Error> {
    todo_port.register(todo)
}
