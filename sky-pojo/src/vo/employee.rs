use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EmployeeLoginVO {
    pub id: i64,
    pub user_name: String,
    pub name: String,
    pub token: String,
}
