use std::{cell::RefCell, sync::Arc};

use crate::todo::Todo;

pub mod action;
pub mod todo;

pub type Todos = Arc<RefCell<Vec<Todo>>>;