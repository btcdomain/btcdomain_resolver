use diesel::{Queryable, AsChangeset};
use rocket::serde::{Serialize, Deserialize};
use crate::schemas::*;

#[derive(Queryable, Deserialize, Serialize, AsChangeset)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = domain_inscription_info)]
pub struct DomainInscriptionInfo {
    pub id: i64,
    pub inscribe_num: i64,
    pub inscribe_id: String,
    pub sat: i64,
    pub domain_name: String,
    pub address: String,
    pub create_time: i64,
    pub update_time: i64,
    pub expire_date: i64,
    pub register_date: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ResolveResp {
    pub proof: Vec<u8>,
    pub address: String,
    pub proof_url: String,
}