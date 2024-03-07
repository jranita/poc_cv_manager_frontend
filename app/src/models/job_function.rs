use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JobFunction {
    pub id: usize,
    pub job_function_name: String,
    pub keyword_list: Vec<usize>,
    pub date_created: NaiveDateTime,
}

impl JobFunction {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewJobFunction {
    pub job_function_name: String,
    pub keyword_list: Vec<usize>,
}
