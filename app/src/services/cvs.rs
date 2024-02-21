
use reqwest;
use reqwest::header::{HeaderMap, ACCEPT, ACCESS_CONTROL_ALLOW_ORIGIN, ORIGIN};
use reqwest::Error;

use crate::models::cv::CV;

pub async fn get_cvs(
    limit: usize,
    offset: usize,
    order_by: String,
    order_direction: String,
    filter: String,
    ) -> Result<Vec<CV>, Error> {

    let url: String = format!("{}cvs?offset={}&limit={}&order_by={}&order_direction={}&filter={}", super::BASE_API_URL, offset, limit, order_by, order_direction, filter);

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
        .json::<Vec<CV>>()
        .await
}

pub async fn get_cv(
    id: usize,
    ) -> Result<CV, Error> {

    let url: String = format!("{}cvs/0?id={}", super::BASE_API_URL, id);

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
        .json::<CV>()
        .await
}
