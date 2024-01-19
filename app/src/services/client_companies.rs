use reqwest;
use reqwest::header::{HeaderMap, ACCEPT, ACCESS_CONTROL_ALLOW_ORIGIN, ORIGIN};
use reqwest::Error;

use crate::models::client_company::ClientCompany;

pub async fn get_client_companies(
    limit: usize,
    offset: usize,
    order_by: String,
    order_direction: String,
    filter: String,
) -> Result<Vec<ClientCompany>, Error> {
    let url: String = format!(
        "{}clients?offset={}&limit={}&order_by={}&order_direction={}&filter={}",
        super::BASE_API_URL,
        offset,
        limit,
        order_by,
        order_direction,
        filter
    );
    // let url: String = format!("{}keywords?offset={}&limit={}", super::BASE_API_URL, offset, limit);
    // let url: String = format!("{}keywords", super::BASE_API_URL);
    let mut headers = HeaderMap::new();
    headers.insert(
        ACCESS_CONTROL_ALLOW_ORIGIN,
        "http://localhost:8080".parse().unwrap(),
    );
    headers.insert(ORIGIN, "http://localhost:8080".parse().unwrap());
    headers.insert(ACCEPT, "application/json".parse().unwrap());

    let client = reqwest::Client::builder().build()?;

    let rows = client
        .get(url)
        .headers(headers)
        .basic_auth("admin@admin.org", Some("password"))
        .send()
        .await?
        .json::<Vec<ClientCompany>>()
        .await;

    rows
}
