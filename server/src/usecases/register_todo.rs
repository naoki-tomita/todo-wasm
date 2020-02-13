use crate::domains::todo::{Todo, Todos, Description};
use crate::error::Error;
use crate::ports::todo_port::TodoPort;

pub fn execute(todo_port: impl TodoPort, text: Description) -> Result<Todo, Error> {
    todo_port.register(text)
}
