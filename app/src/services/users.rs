
use reqwest;
use reqwest::header::{HeaderMap, ACCEPT, ACCESS_CONTROL_ALLOW_ORIGIN, ORIGIN};
use reqwest::Error;

use crate::models::user::User;

pub async fn get_users(
    limit: usize,
    offset: usize,
    order_by: String,
    order_direction: String,
    filter: String,
    ) -> Result<Vec<User>, Error> {

    let url: String = format!("{}users?offset={}&limit={}&order_by={}&order_direction={}&filter={}", super::BASE_API_URL, offset, limit, order_by, order_direction, filter);

    let mut headers = HeaderMap::new();
    headers.insert(
        ACCESS_CONTROL_ALLOW_ORIGIN,
        "http://localhost:8080".parse().unwrap(),
    );
    headers.insert(ORIGIN, "http://localhost:8080".parse().unwrap());
    headers.insert(ACCEPT, "application/json".parse().unwrap());

    let client = reqwest::Client::builder().build()?;

    client
        .get(url)
        .headers(headers)
        .basic_auth("admin@admin.org", Some("password"))
        .send()
        .await?
        .json::<Vec<User>>()
        .await
}

pub async fn get_user(
    id: usize,
    ) -> Result<User, Error> {

    let url: String = format!("{}users/0?id={}", super::BASE_API_URL, id);

    let mut headers = HeaderMap::new();
    headers.insert(
        ACCESS_CONTROL_ALLOW_ORIGIN,
        "http://localhost:8080".parse().unwrap(),
    );
    headers.insert(ORIGIN, "http://localhost:8080".parse().unwrap());
    headers.insert(ACCEPT, "application/json".parse().unwrap());

    let client = reqwest::Client::builder().build()?;

    client
        .get(url)
        .headers(headers)
        .basic_auth("admin@admin.org", Some("password"))
        .send()
        .await?
        .json::<User>()
        .await
}
