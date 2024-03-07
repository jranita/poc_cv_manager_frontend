use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClientCompany {
    pub id: usize,
    pub company_name: String,
    pub date_created: NaiveDateTime,
}

// TODO: limit and offset are needed for sorting and pagination
impl ClientCompany {}

#[derive(Clone, Debug, Deserialize)]
pub struct NewClientCompany {
    pub company_name: String,
}
