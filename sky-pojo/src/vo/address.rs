use serde::Serialize;

use crate::entities::address_book;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddressVO {
    pub id: i64,
    pub user_id: i64,
    pub consignee: String,
    pub sex: String,
    pub phone: String,
    pub province_code: String,
    pub province_name: String,
    pub city_code: String,
    pub city_name: String,
    pub district_code: String,
    pub district_name: String,
    pub detail: String,
    pub label: String,
    pub is_default: i16,
}

impl From<address_book::Model> for AddressVO {
    fn from(value: address_book::Model) -> Self {
        Self {
            id: value.id,
            user_id: value.user_id,
            consignee: value.consignee.unwrap_or_default(),
            sex: value.sex.unwrap_or_default(),
            phone: value.phone,
            province_code: value.province_code.unwrap_or_default(),
            province_name: value.province_name.unwrap_or_default(),
            city_code: value.city_code.unwrap_or_default(),
            city_name: value.city_name.unwrap_or_default(),
            district_code: value.district_code.unwrap_or_default(),
            district_name: value.district_name.unwrap_or_default(),
            detail: value.detail.unwrap_or_default(),
            label: value.label.unwrap_or_default(),
            is_default: value.is_default,
        }
    }
}
