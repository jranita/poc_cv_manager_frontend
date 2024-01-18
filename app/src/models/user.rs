use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub pass: String,
    pub role: String,
    pub cv_id_list: Vec<i32>,
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
    pub user_id: i32,
    pub old_password: String,
    pub new_password: String,
}

#[derive(Clone, Debug)]
pub struct CurrentUser {
    pub id: i32,
    pub role: String,
}
