use crate::domains::todo::{Todo, Todos};
use crate::error::Error;

pub trait TodoPort {
    fn get_list(&self) -> Result<Todos, Error>;
    fn register(&self, todo: Todo) -> Result<Todo, Error>;
}
