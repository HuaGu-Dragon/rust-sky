use serde::Deserialize;

#[derive(Deserialize)]
pub struct EmployeeLoginDto {
    pub username: String,
    pub password: String,
}
