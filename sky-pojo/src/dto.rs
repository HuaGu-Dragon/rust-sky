use rust_decimal::{Decimal, prelude::FromPrimitive};
use serde::{
    Deserialize, Deserializer,
    de::{self, Unexpected},
};

pub mod category;
pub mod dish;
pub mod employee;
pub mod setmeal;
pub mod shopping_cart;
pub mod user;

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

pub fn deserialize<'de, D>(des: D) -> Result<Decimal, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Helper {
        F64(f64),
        I64(i64),
        U64(u64),
        Str(String),
    }

    match Helper::deserialize(des)? {
        Helper::F64(v) => {
            Decimal::from_f64(v).ok_or_else(|| de::Error::custom("Invalid f64 for Decimal"))
        }
        Helper::I64(v) => Ok(Decimal::from(v)),
        Helper::U64(v) => Ok(Decimal::from(v)),
        Helper::Str(s) => s
            .parse::<Decimal>()
            .map_err(|e| de::Error::custom(format!("ParseDecimalError: {e}"))),
    }
}
