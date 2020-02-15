use crate::domains::todo::{Description, Id, IdentifiedTodo, Stat};
use crate::error::Error;
use crate::ports::todo_port::TodoPort;

pub fn execute(
    todo_port: impl TodoPort,
    id: Id,
    text: Option<Description>,
    done: Option<Stat>,
) -> Result<IdentifiedTodo, Error> {
    todo_port.update(id, text, done)
}
