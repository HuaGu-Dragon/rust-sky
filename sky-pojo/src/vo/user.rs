use serde::Serialize;

#[derive(Serialize)]
pub struct UserLoginVo {
    pub id: i64,
    pub openid: String,
    pub token: String,
}
