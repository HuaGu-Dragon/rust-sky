use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct EmployeeLoginVO {
    pub id: i64,
    pub username: String,
    pub name: String,
    pub token: String,
}
