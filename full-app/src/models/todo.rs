#[derive(Debug, Clone)]
pub struct Todo {
    pub id: usize,
    pub text: String,
    pub done: bool,
}

#[derive(Debug, Clone)]
pub struct Todos {
    pub list: Vec<Todo>,
}

impl Todos {
    pub fn new() -> Self {
        Todos { list: vec![] }
    }

    pub fn find(&self, id: usize) -> Option<Todo> {
        match self.list.iter().find(|item| item.id == id) {
            Some(item) => {
                Some(item.clone())
            },
            None => None
        }

    }
}
