use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserLoginDto {
    pub code: String,
}
