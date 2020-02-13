use crate::domains::todo::Todos;
use crate::error::Error;
use crate::ports::todo_port::TodoPort;

pub fn execute(todo_port: impl TodoPort) -> Result<Todos, Error> {
    todo_port.get_list()
}
