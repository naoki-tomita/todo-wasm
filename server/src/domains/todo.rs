pub struct Id(pub usize);
pub struct Description(pub String);

#[derive(PartialEq)]
pub enum Stat {
    Done,
    UnDone,
}

pub struct Todo {
    pub text: Description,
    pub done: Stat,
}

impl Todo {
    pub fn new(text: String, done: bool) -> Self {
        Todo {
            text: Description(text),
            done: if done { Stat::Done } else { Stat::UnDone },
        }
    }
}

pub struct Todos {
    pub values: Vec<Todo>,
}

impl Todos {
    pub fn new(values: Vec<Todo>) -> Self {
        Todos { values }
    }
}

pub struct IdentifiedTodo {
    pub id: Id,
    pub todo: Todo,
}

impl IdentifiedTodo {
    pub fn new(id: usize, text: String, done: bool) -> Self {
        IdentifiedTodo {
            id: Id(id),
            todo: Todo::new(text, done),
        }
    }
}

pub struct IdentifiedTodos {
    pub values: Vec<IdentifiedTodo>,
}

impl IdentifiedTodos {
    pub fn new(values: Vec<IdentifiedTodo>) -> Self {
        IdentifiedTodos { values }
    }
}
