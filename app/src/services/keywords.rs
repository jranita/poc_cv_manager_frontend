use crate::models::keyword::Keyword;
use reqwest;
use reqwest::Error;
use reqwest::header::{HeaderMap, ACCESS_CONTROL_ALLOW_ORIGIN};

pub async fn get_keywords(
    limit: usize,
    offset: usize,
    order_by: String,
    order_direction: String,
    filter: String,
) -> Result<&'static str, Error> {
// ) -> Result<Vec<Keyword>, reqwest::Error> {

    // let url: String = format!("{}keywords?offset={}&limit={}&order_by={}&order_direction={}&filter={}", super::BASE_API_URL, offset, limit, order_by, order_direction, filter);
    // let url: String = format!("{}keywords?offset={}&limit={}", super::BASE_API_URL, offset, limit);
    let url: String = format!("{}keywords", super::BASE_API_URL);
    let mut headers = HeaderMap::new();
    headers.insert(ACCESS_CONTROL_ALLOW_ORIGIN, "http://localhost:8080".parse().unwrap());

    let client = reqwest::Client::builder()
        .build()?;

    let rows = client.get(url)
        .headers(headers)
        .basic_auth("admin@admin.org", Some("password"))
        .send().await?;

    println!("{:?}", rows);

    // let keywords_list = rows
    //     .iter()
    //     .map(|r| Keyword {
    //         id: r.get("id"),
    //         keyword_name: r.get("keyword_name"),
    //         // date_created: r.get<chrono::Utc>("date_created").date_naive(),
    //         date_created: r.get("date_created"),
    //     })
    //     .collect::<Vec<Keyword>>();
    // println!("{:?}", keywords_list[0]);

    Ok("eee")
}
