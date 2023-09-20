// tweets/:id -> PUT: actualiza un tweet
// tweets/:id -> DELETE: elimina un tweet
// tweets/:id/likes -> GET: crea un like en un tweet
// tweets/:id/likes -> DELETE: elimina un like en un tweet
// tweets/:id/likes -> POST: elimina un like en un tweet
#[macro_use]
extern crate diesel;

use actix_web::web::Data;
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use std::env;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::MysqlConnection;
use dotenv::dotenv;

mod likes;
mod tweets;
mod constants;
mod schema;

async fn say_hello() -> impl Responder {
    HttpResponse::Ok().body("Hola mundo")
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    let pool = Pool::builder().build(manager).expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
        .app_data(Data::new(pool.clone()))
        .route("/", web::get().to(say_hello))
        .service(tweets::get_tweets)
        .service(tweets::create_tweets)
        .service(tweets::get_tweet_by_id)
        .service(likes::get_tweet_likes)
        .service(likes::remove_like_tweet)
        .service(likes::make_like_tweet)
        .service(likes::remove_like_tweet)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
