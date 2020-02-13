#[derive(Debug, Clone)]
pub struct Todo {
    pub text: String,
    pub done: bool,
}

impl Todo {
    pub fn switch(&mut self) -> Self {
        Todo {
            text: self.text.clone(),
            done: !self.done,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Todos {
    pub list: Vec<Todo>,
}

impl Todos {
    pub fn new() -> Self {
        Todos { list: vec![] }
    }

    pub fn push(&mut self, text: String) {
        self.list.push(Todo { text, done: false })
    }

    pub fn switch(&mut self, index: usize) {
        self.list[index] = self.list[index].switch()
    }

    pub fn with_index(&self) -> Vec<(Todo, usize)> {
        let mut result: Vec<(Todo, usize)> = vec![];
        for i in (0..).take(self.list.len()) {
            result.push((self.list[i].clone(), i))
        }
        result
    }
}
