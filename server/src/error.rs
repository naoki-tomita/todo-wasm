use crate::drivers::todo_driver::TodoEntity;
use std::sync::RwLockReadGuard;
use std::sync::PoisonError;
use failure::_core::fmt;
use failure::_core::fmt::Display;
use failure::{Context, Fail};

#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}

// impl Fail for Error {
//     fn name(&self) -> Option<&str> {
//         self.inner.name()
//     }

//     fn cause(&self) -> Option<&Fail> {
//         self.inner.cause()
//     }

//     fn backtrace(&self) -> Option<&Backtrace> {
//         self.inner.backtrace()
//     }
// }

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl Error {
    // pub fn kind(&self) -> &ErrorKind {
    //     self.inner.get_context()
    // }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(inner: Context<ErrorKind>) -> Error {
        Error { inner }
    }
}

impl From<PoisonError<RwLockReadGuard<'_, Vec<TodoEntity>>>> for Error {
    fn from(_inner: PoisonError<RwLockReadGuard<'_, Vec<TodoEntity>>>) -> Error {
        Error { inner: Context::new(ErrorKind::NotFound) }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Fail)]
pub enum ErrorKind {
    #[fail(display = "Not Found.")]
    NotFound,
}
