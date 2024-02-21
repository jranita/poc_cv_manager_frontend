use crate::models::keyword::Keyword;
use reqwest;
use reqwest::header::{HeaderMap, ACCEPT, ACCESS_CONTROL_ALLOW_ORIGIN, ORIGIN};
use reqwest::Error;

pub async fn get_keywords(
    limit: usize,
    offset: usize,
    order_by: String,
    order_direction: String,
    filter: String,
    ) -> Result<Vec<Keyword>, Error> {

    let url: String = format!("{}keywords?offset={}&limit={}&order_by={}&order_direction={}&filter={}", super::BASE_API_URL, offset, limit, order_by, order_direction, filter);

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
        .json::<Vec<Keyword>>()
        .await

}

pub async fn get_keyword(
    id: usize,
    ) -> Result<Keyword, Error> {

    let url: String = format!("{}keywords/0?id={}", super::BASE_API_URL, id);

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
        .json::<Keyword>()
        .await

}

