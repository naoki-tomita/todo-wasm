use crate::domains::todo::{Description, IdentifiedTodo};
use crate::error::Error;
use crate::ports::todo_port::TodoPort;

pub fn execute(todo_port: impl TodoPort, text: Description) -> Result<IdentifiedTodo, Error> {
    todo_port.register(text)
}
