use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CV {
    pub id: usize,
    pub user_id: usize,
    pub cv_name: String,
    pub file_name: String,
    pub keyword_list: Vec<usize>,
    pub target_companies: Vec<usize>,
    pub target_job_functions: Vec<usize>,
    pub date_created: NaiveDateTime,
}

impl CV {}

#[derive(Clone, Debug, Deserialize)]
pub struct NewCV {
    pub user_id: usize,
    pub cv_name: String,
    pub file_name: String,
    pub keyword_list: Vec<usize>,
    pub target_companies: Vec<usize>,
    pub target_job_functions: Vec<usize>,
}
