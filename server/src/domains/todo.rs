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
