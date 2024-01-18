use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CV {
    pub id: i32,
    pub user_id: i32,
    pub cv_name: String,
    pub file_name: String,
    pub keyword_list: Vec<i32>,
    pub target_companies: Vec<i32>,
    pub target_job_functions: Vec<i32>,
    pub date_created: NaiveDateTime,
}

impl CV {}

#[derive(Clone, Debug, Deserialize)]
pub struct NewCV {
    pub user_id: i32,
    pub cv_name: String,
    pub file_name: String,
    pub keyword_list: Vec<i32>,
    pub target_companies: Vec<i32>,
    pub target_job_functions: Vec<i32>,
}
