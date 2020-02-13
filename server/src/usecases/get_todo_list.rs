use crate::domains::todo::IdentifiedTodos;
use crate::error::Error;
use crate::ports::todo_port::TodoPort;

pub fn execute(todo_port: impl TodoPort) -> Result<IdentifiedTodos, Error> {
    todo_port.get_list()
}
