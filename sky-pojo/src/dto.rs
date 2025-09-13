use serde::{
    Deserialize, Deserializer,
    de::{self, Unexpected},
};

pub mod category;
pub mod dish;
pub mod employee;
pub mod setmeal;

#[derive(Debug, Deserialize)]
pub struct StateQuery {
    pub id: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryDelete {
    pub ids: String,
}

// uri=/admin/dish/page?page=1&pageSize=10&status=
// what is that fucking request uri???
pub fn empty_string_as_none<'de, D>(deserializer: D) -> Result<Option<i32>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Opt {
        I(i32),
        S(String),
        N,
    }

    match Opt::deserialize(deserializer)? {
        Opt::I(i) => Ok(Some(i)),
        Opt::S(s) if s.trim().is_empty() => Ok(None),
        Opt::S(s) => s
            .parse::<i32>()
            .map(Some)
            .map_err(|_| de::Error::invalid_value(Unexpected::Str(&s), &"an integer or empty")),
        Opt::N => Ok(None),
    }
}
