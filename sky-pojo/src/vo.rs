use serde::Serialize;

pub mod dish;
pub mod employee;
pub mod flavor;
pub mod setmeal;
pub mod user;

#[derive(Debug, Clone, Serialize)]
pub struct Page<T> {
    total: i64,
    records: Vec<T>,
}

impl<T> Page<T> {
    pub fn new(total: i64, records: Vec<T>) -> Self {
        Self { total, records }
    }
}
