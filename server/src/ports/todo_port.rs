use crate::domains::todo::{Description, Id, IdentifiedTodo, IdentifiedTodos, Stat};
use crate::error::Error;

pub trait TodoPort {
    fn get_list(&self) -> Result<IdentifiedTodos, Error>;
    fn register(&self, text: Description) -> Result<IdentifiedTodo, Error>;
    fn update(
        &self,
        id: Id,
        text: Option<Description>,
        done: Option<Stat>,
    ) -> Result<IdentifiedTodo, Error>;
}
