use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
// use futures::future::join_all;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Keyword {
    pub id: i32,
    pub keyword_name: String,
    pub date_created: NaiveDateTime,
}

impl Keyword {}

#[derive(Clone, Debug, Deserialize)]
pub struct NewKeyword {
    pub keyword_name: String,
}
