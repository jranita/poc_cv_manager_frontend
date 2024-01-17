use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};


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

impl User {
    // pub async fn get_users(
    //     // depot: &mut Depot,
    //     limit: usize,
    //     offset: usize,
    //     order_by: String,
    //     order_direction: String,
    //     filter: String,
    // ) -> Result<Vec<User>, Error> {
    //     println!("28     Get_users()");

    //     let current_user: &CurrentUser = depot
    //         .get("currentuser")
    //         .expect("missing current user in depot");
    //     if current_user.role != "admin" {
    //         let result = Self::get_user(depot, current_user.id).await;
    //         return Ok(vec![result.unwrap()]);
    //     }

    //     let query: String = format!("SELECT id, firstname, lastname, date_created FROM users {} ORDER BY {} {} OFFSET {} LIMIT {}", filter, order_by, order_direction, offset, limit);

    //     let rows = sqlx::query(&query)
    //         .fetch_all(get_postgres())
    //         .await
    //         .map_err(|e| {
    //             tracing::error!("Failed to execute query: {:?}", e);
    //             anyhow::anyhow!("Failed to execute query")
    //         })?;

    //     let users_list = rows
    //         .iter()
    //         .map(|r| User {
    //             id: r.get("id"),
    //             firstname: r.get("firstname"),
    //             lastname: r.get("lastname"),
    //             date_created: r.get("date_created"),
    //             email: "".to_owned(),
    //             role: "".to_owned(),
    //             pass: "".to_owned(),
    //             cv_id_list: vec![],
    //         })
    //         .collect::<Vec<User>>();

    //     Ok(users_list)
    // }

    // pub async fn get_user(depot: &mut Depot, mut target_id: i32) -> Result<User, Error> {
    //     let current_user: &CurrentUser = depot
    //         .get("currentuser")
    //         .expect("missing current user in depot");
    //     if current_user.id != target_id && current_user.role != "admin" {
    //         println!("Will not be allowed, a user can only get his own information");
    //         target_id = current_user.id;
    //     }

    //     let query_string = format!("SELECT * from users where id={}", target_id);

    //     //TODO use query_as
    //     let row = sqlx::query(&query_string)
    //         .fetch_one(get_postgres())
    //         .await
    //         .map_err(|e| {
    //             tracing::error!("Failed to execute query: {:?}", e);
    //             anyhow::anyhow!("Failed to execute query")
    //         })?;

    //     let user = User {
    //         id: row.get("id"),
    //         firstname: row.get("firstname"),
    //         lastname: row.get("lastname"),
    //         date_created: row.get("date_created"),
    //         email: row.get("email"),
    //         pass: row.get("password"),
    //         role: row.get("role"),
    //         cv_id_list: row.get("cv_id_list"),
    //     };

    //     Ok(user)
    // }

//     pub async fn get_user_by_email(target_email: String) -> Result<User, Error> {
//         let query_string = format!("SELECT * from users where email='{}'", target_email);

//         //TODO use query_as
//         let row = sqlx::query(&query_string)
//             .fetch_one(get_postgres())
//             .await
//             .map_err(|e| {
//                 tracing::error!("Failed to execute query: {:?}", e);
//                 anyhow::anyhow!("92 Failed to execute query")
//             })?;

//         if row.is_empty() {
//             println!("Empty row");
//         }

//         let user = User {
//             id: row.get("id"),
//             firstname: row.get("firstname"),
//             lastname: row.get("lastname"),
//             date_created: row.get("date_created"),
//             email: row.get("email"),
//             pass: row.get("password"),
//             role: row.get("role"),
//             cv_id_list: row.get("cv_id_list"),
//         };

//         // println!("106 {}\n{}", user.pass, user.pass);

//         Ok(user)
//     }

//     pub async fn insert_user(c: NewUser) -> Result<User, Error> {
//         println!(
//             "56     insert_user() {:?} {:?}",
//             c,
//             NaiveDateTime::default()
//         );

//         let query: String = format!(
//             "INSERT INTO users (email, firstname, lastname, password, role) VALUES ('{}', '{}', '{}', '{}', '{}') RETURNING *",
//             c.email, c.firstname, c.lastname, c.pass, c.role
//         );
//         println!("59     query {:?}", query);

//         let inserted = sqlx::query(&query)
//             .fetch_one(get_postgres())
//             .await
//             .map_err(|e| {
//                 tracing::error!("Failed to execute insert query: {:?}", e);
//                 anyhow::anyhow!("Failed to insert record")
//             })?;

//         let hashed_password = crate::authentication::hash_password(inserted.get("password"))?;

//         Ok(User {
//             id: inserted.get("id"),
//             firstname: inserted.get("firstname"),
//             lastname: inserted.get("lastname"),
//             email: inserted.get("email"),
//             pass: hashed_password,
//             date_created: inserted.get("date_created"),
//             cv_id_list: inserted.get("cv_id_list"),
//             role: inserted.get("role"),
//         })
//     }

//     pub async fn update_user(c: User) -> Result<User, Error> {
//         println!("101     update_user() {:?}", c);

//         let cvs: String = number_vec_to_string(&c.cv_id_list);
//         let query: String = format!(
//             "UPDATE users SET firstname='{}', lastname='{}', email='{}' password='{}' role='{}' cv_id_list='{}' WHERE id='{}'",
//             c.firstname, c.lastname, c.email, c.pass, c.role, cvs, c.id
//         );
//         println!("133     query {:?}", query);

//         let updated = sqlx::query(&query)
//             .execute(get_postgres())
//             .await
//             .map(|r| r.rows_affected())
//             .map_err(|e| {
//                 tracing::error!("Failed to execute update query: {:?}", e);
//                 anyhow::anyhow!("Failed to update record")
//             })?;

//         // TODO improve error creation/handling
//         if updated == 0 {
//             tracing::error!("Failed update query: probably the ID does not exist");
//             return Err(Error::from(anyhow::anyhow!(
//                 "Failed update query: probably the ID does not exist"
//             )));
//         }

//         Ok(c)
//     }

//     pub async fn change_user_password(depot: &mut Depot, c: PasswordStruct) -> Result<User, Error> {
//         let hashed_new_password = crate::authentication::hash_password(c.clone().new_password)?;
//         let target_user = Self::get_user(depot, c.user_id).await?;

//         let _ = authentication::authorize_user(
//             &target_user,
//             &authentication::Credentials {
//                 email: target_user.clone().email,
//                 password: c.old_password,
//             },
//         )
//         .map_err(|e| {
//             tracing::error!("Failed to execute query: {:?}", e);
//             anyhow::anyhow!("Failed to update password")
//         })?;

//         let query: String = format!(
//             "UPDATE users SET password='{}' WHERE id='{}' returning *",
//             hashed_new_password, c.user_id
//         );

//         let updated = sqlx::query(&query)
//             .execute(get_postgres())
//             .await
//             .map(|r| r.rows_affected())
//             .map_err(|e| {
//                 tracing::error!("Failed to execute update query: {:?}", e);
//                 anyhow::anyhow!("Failed to update record")
//             })?;

//         // TODO improve error creation/handling
//         if updated == 0 {
//             tracing::error!("Failed update query: probably the ID does not exist");
//             return Err(Error::from(anyhow::anyhow!(
//                 "Failed update query: probably the ID does not exist"
//             )));
//         }

//         Ok(target_user)
//     }

//     pub async fn delete_user(id: i32) -> Result<User, Error> {
//         println!("130     delete_user() {:?}", id);

//         let query: String = format!("DELETE FROM users WHERE id='{}'", id);
//         println!("133     query {:?}", query);

//         let deleted = sqlx::query(&query)
//             .execute(get_postgres())
//             .await
//             .map(|r| r.rows_affected())
//             .map_err(|e| {
//                 tracing::error!("Failed to execute delete query: {:?}", e);
//                 anyhow::anyhow!("Failed to delete record")
//             })?;

//         // TODO improve error creation/handling
//         if deleted == 0 {
//             tracing::error!("Failed delete record: probably the ID does not exist");
//             return Err(Error::from(anyhow::anyhow!(
//                 "Failed delete query: probably the ID does not exist"
//             )));
//         }

//         let ccc = User {
//             id,
//             date_created: NaiveDateTime::default(),
//             firstname: "firstname".to_string(),
//             lastname: "lastname".to_string(),
//             email: "lastname".to_string(),
//             pass: "lastname".to_string(),
//             role: "lastname".to_string(),
//             cv_id_list: vec![],
//         };

//         Ok(ccc)
//     }
// }

// #[derive(Clone, Debug, Deserialize, ToSchema)]
// pub struct NewUser {
//     pub firstname: String,
//     pub lastname: String,
//     pub email: String,
//     pub pass: String,
//     pub role: String,
// }

// #[derive(Clone, Debug, Deserialize, ToSchema)]
// pub struct PasswordStruct {
//     pub user_id: i32,
//     pub old_password: String,
//     pub new_password: String,
// }

// #[derive(Clone, Debug)]
// pub struct CurrentUser {
//     pub id: i32,
//     pub role: String,
}
