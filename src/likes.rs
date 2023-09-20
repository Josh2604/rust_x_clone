use actix_web::{get, delete, post, HttpResponse};
use actix_web::web::Path;

use crate::constants::APPLICATION_JSON;

#[get("/tweets/{id}/likes")]
pub async fn get_tweet_likes(path: Path<(String,)>)-> HttpResponse {
    // TODO: Getting tweets likes
    let likes = 100;

    HttpResponse::Ok()
    .content_type(APPLICATION_JSON)
    .json(likes)
}

#[post("/tweets/{id}/likes")]
pub async fn make_like_tweet(path: Path<(String,)>)-> HttpResponse {
    // TODO: Do like tweet
    let like = "Ok";

    HttpResponse::Created()
    .content_type(APPLICATION_JSON)
    .json(like)
}

#[delete("/tweets/{id}/likes")]
pub async fn remove_like_tweet(path: Path<(String,)>)-> HttpResponse {
    // TODO: decrease likes for tweet
    // let result = "Ok";

    HttpResponse::NoContent() //200, 204
    .content_type(APPLICATION_JSON)
    .await
    .unwrap()
}