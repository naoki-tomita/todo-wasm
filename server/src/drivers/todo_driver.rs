use crate::error::Error;
use std::collections::HashMap;
use std::sync::RwLock;

lazy_static! {
    // RwLockでGlobalな変数に書き込み、読み込みができるようにする
    static ref DATUM: RwLock<Vec<TodoEntity>> = {
        RwLock::new(vec![])
    };
}

pub fn get_todos() -> Result<Vec<TodoEntity>, Error> {
    // cloneしないと、DATUMを消費してしまう？
    // readで読み取り専用でアクセスできる
    Ok(DATUM.read().unwrap().clone())
}

pub fn register(text: String, done: bool) -> Result<TodoEntity, Error> {
    // writeで書き込み可で(mutとして)アクセスできる
    let mut list = DATUM.write().unwrap();
    // これはなんで消費しないんだっけ？ len(&self) だから？
    // WriteGuardという型はどうやらポインタみたいなもんらしいので、実際にアクセスするには * をつけて実体にアクセスする必要があるっぽい。
    let id = (*list).len() + 1;
    let todo = TodoEntity { text, done, id };
    // このあと使うので、cloneする。
    (*list).push(todo.clone());
    // ここで消費しきる。
    Ok(todo)
}

pub fn test_clear() {
    // writeで書き込み可で(mutとして)アクセスできる
    let mut list = DATUM.write().unwrap();
    (*list).clear();
    (*list).push(TodoEntity {
        id: 1,
        text: "Todo1".to_string(),
        done: false,
    });
    (*list).push(TodoEntity {
        id: 2,
        text: "Hello1".to_string(),
        done: false,
    });
    (*list).push(TodoEntity {
        id: 3,
        text: "Good1".to_string(),
        done: false,
    });
}

pub fn test_register_data() {
    // writeで書き込み可で(mutとして)アクセスできる
    let mut list = DATUM.write().unwrap();
    (*list).clear();
    (*list).push(TodoEntity {
        id: 1,
        text: "Todo1".to_string(),
        done: false,
    });
    (*list).push(TodoEntity {
        id: 2,
        text: "Hello1".to_string(),
        done: false,
    });
    (*list).push(TodoEntity {
        id: 3,
        text: "Good1".to_string(),
        done: false,
    });
}


#[derive(Clone)]
pub struct TodoEntity {
    pub id: usize,
    pub text: String,
    pub done: bool,
}
