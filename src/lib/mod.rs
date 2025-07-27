use std::{cell::RefCell, sync::Arc};

pub mod action;
pub mod todo;
pub mod errors;

pub type Todos = Arc<RefCell<Vec<todo::Todo>>>;