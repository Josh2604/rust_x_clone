use actix_web::{get, post, HttpResponse};
use actix_web::web::{Path, Data};
use chrono::{NaiveDateTime, Utc};
use diesel::{MysqlConnection, RunQueryDsl, Queryable, Insertable};
use diesel::r2d2::{Pool, ConnectionManager};
use diesel::sql_types::{Text, Timestamp, BigInt};
use uuid::Uuid;
use serde::{Deserialize, Serialize};

use diesel::prelude::*;
// use diesel::sql_types::DateTime;

use crate::constants::APPLICATION_JSON;

use super::schema::tweets;

// #[derive(Queryable, Insertable, Serialize, Deserialize)]
// #[table_name = "tweets"]
// struct Tweet {
//     id: Uuid,
//     created_at: NaiveDateTime, //  naive = ingenuo. ISO 8601, sin zona horaria
//     // created_at: Timestamp,
//     message: String,
// }
#[derive(Queryable, Insertable, Serialize, Deserialize)]
struct Tweet {
    // id: Uuid,
    // id: i64,
    created_at: NaiveDateTime,
    message: String,
}

impl Tweet {
  fn new(message: String) -> Self {
      Self {
          // id: Uuid::new_v4().to_string().as_bytes().,
          created_at: Utc::now().naive_utc(),
          message,
      }
  }
}

// tweets -> GET: obtiene los tweets
#[get("/tweets")]
pub async fn get_tweets()-> HttpResponse {
    // TODO: Get tweets
    let tweets = ["tweet 1: 1", "tweet2: 2", "tweet3: 3"];

    HttpResponse::Ok()
    .content_type(APPLICATION_JSON)
    .json(tweets)
}

// tweets -> POST: crea un nuevo tweet
#[post("/tweets")]
pub async fn create_tweets(req_body: String, pool: Data<Pool<ConnectionManager<MysqlConnection>>>)-> HttpResponse {
    // TODO: Create tweets
    print!("req_body: {}", req_body);

    let nuevo_tweet = Tweet::new(req_body);
    let mut conn = pool
        .get()
        .expect(" No pude obtener conexión a la base de datos");

    diesel::insert_into(tweets::table)
        .values(&nuevo_tweet)
        .execute(&mut conn)
        .expect("Error al insertar tweet");

    HttpResponse::Created()
        .content_type(APPLICATION_JSON)
        .json(&nuevo_tweet)
}

// tweets/:id -> GET: obtiene un tweet
#[get("/tweets/{id}")]
pub async fn get_tweet_by_id(path: Path<(String,)>)-> HttpResponse {
    // TODO: Get tweet with id
    let tweet = format!("This is the id {:?}", path.0);

    HttpResponse::Ok()
    .content_type(APPLICATION_JSON)
    .json(tweet)
}