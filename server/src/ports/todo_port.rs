use crate::domains::todo::{Todo, Todos, Description};
use crate::error::Error;

pub trait TodoPort {
    fn get_list(&self) -> Result<Todos, Error>;
    fn register(&self, text: Description) -> Result<Todo, Error>;
}
