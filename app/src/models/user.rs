use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: usize,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub pass: String,
    pub role: String,
    pub cv_id_list: Vec<usize>,
    pub date_created: NaiveDateTime,
}

impl User {}

#[derive(Clone, Debug, Deserialize)]
pub struct NewUser {
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub pass: String,
    pub role: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PasswordStruct {
    pub user_id: usize,
    pub old_password: String,
    pub new_password: String,
}

#[derive(Clone, Debug)]
pub struct CurrentUser {
    pub id: usize,
    pub role: String,
}
