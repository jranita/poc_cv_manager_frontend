use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use futures::future::join_all;

pub static API_URL: &str = "https://localhost:5800/keywords";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Keyword {
    pub id: i32,
    pub keyword_name: String,
    pub date_created: NaiveDateTime,
}

impl Keyword {

    // pub async fn get_keyword(target_id: i32) -> Result<Keyword, Error> {
    //     let query_string = format!("SELECT * from keywords where id={}", target_id);

    //     //TODO use query_as
    //     let row = sqlx::query(&query_string)
    //         .fetch_one(get_postgres())
    //         .await
    //         .map_err(|e| {
    //             tracing::error!("Failed to execute query: {:?}", e);
    //             anyhow::anyhow!("Failed to execute query")
    //         })?;

    //     // println!("{:?}", rows[0].columns());

    //     let keyword = Keyword {
    //         id: row.get("id"),
    //         keyword_name: row.get("keyword_name"),
    //         date_created: row.get("date_created"),
    //     };

    //     Ok(keyword)
    // }

    // pub async fn insert_keyword(c: NewKeyword) -> Result<Keyword, Error> {
    //     println!(
    //         "56     insert_keyword() {:?} {:?}",
    //         c,
    //         NaiveDateTime::default()
    //     );

    //     let query: String = format!(
    //         "INSERT INTO keywords (keyword_name) VALUES ('{}') RETURNING *",
    //         c.keyword_name
    //     );
    //     println!("59     query {:?}", query);

    //     let inserted = sqlx::query(&query)
    //         .fetch_one(get_postgres())
    //         .await
    //         .map_err(|e| {
    //             tracing::error!("Failed to execute insert query: {:?}", e);
    //             anyhow::anyhow!("Failed to insert record")
    //         })?;

    //     Ok(Keyword {
    //         id: inserted.get("id"),
    //         keyword_name: inserted.get("keyword_name"),
    //         date_created: inserted.get("date_created"),
    //     })
    // }

    // pub async fn update_keyword(c: Keyword) -> Result<Keyword, Error> {
    //     println!("101     update_keyword() {:?}", c);

    //     let query: String = format!(
    //         "UPDATE keywords SET keyword_name='{}' WHERE id='{}'",
    //         c.keyword_name, c.id
    //     );
    //     println!("108     query {:?}", query);

    //     let updated = sqlx::query(&query)
    //         .execute(get_postgres())
    //         .await
    //         .map(|r| r.rows_affected())
    //         .map_err(|e| {
    //             tracing::error!("Failed to execute update query: {:?}", e);
    //             anyhow::anyhow!("Failed to update record")
    //         })?;

    //     // TODO improve error creation/handling
    //     if updated == 0 {
    //         tracing::error!("Failed update query: probably the ID does not exist");
    //         return Err(Error::from(anyhow::anyhow!(
    //             "Failed update query: probably the ID does not exist"
    //         )));
    //     }

    //     Ok(c)
    // }

    // pub async fn delete_keyword(id: i32) -> Result<Keyword, Error> {
    //     println!("130     delete_keyword() {:?}", id);

    //     let query: String = format!("DELETE FROM keywords WHERE id='{}'", id);
    //     println!("133     query {:?}", query);

    //     let deleted = sqlx::query(&query)
    //         .execute(get_postgres())
    //         .await
    //         .map(|r| r.rows_affected())
    //         .map_err(|e| {
    //             tracing::error!("Failed to execute delete query: {:?}", e);
    //             anyhow::anyhow!("Failed to delete record")
    //         })?;

    //     // TODO improve error creation/handling
    //     if deleted == 0 {
    //         tracing::error!("Failed delete record: probably the ID does not exist");
    //         return Err(Error::from(anyhow::anyhow!(
    //             "Failed delete query: probably the ID does not exist"
    //         )));
    //     }

    //     let ccc = Keyword {
    //         id,
    //         date_created: NaiveDateTime::default(),
    //         keyword_name: "company_name".to_string(),
    //     };

    //     Ok(ccc)
    // }
}

#[derive(Clone, Debug, Deserialize)]
pub struct NewKeyword {
    pub keyword_name: String,
}
