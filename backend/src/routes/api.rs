use actix_web::{web, HttpResponse, Responder};
use serde_json::json;

use crate::db::redis_client::RedisClient;
use crate::models::{Post, User};
use crate::handlers::{user_handlers, post_handlers};

// This function configures the routes for our application
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/users", web::post().to(user_handlers::create_user))
            .route("/users/{id}", web::get().to(user_handlers::get_user))
            .route("/posts", web::post().to(post_handlers::create_post))
            .route("/posts/{id}", web::get().to(post_handlers::get_post))
            // Add more routes as needed
    );
}

// The following are placeholder functions for the actual handlers that should be implemented in `handlers` module
// They are here just to give an idea of what the handlers might look like

// mod handlers {
//     use super::*;

//     pub mod user_handlers {
//         use super::*;

//         pub async fn create_user(redis_client: web::Data<RedisClient>, user: web::Json<User>) -> impl Responder {
//             match redis_client.save_user(&user.into_inner()) {
//                 Ok(_) => HttpResponse::Created().json("User created"),
//                 Err(_) => HttpResponse::InternalServerError().json("Error creating user"),
//             }
//         }

//         pub async fn get_user(redis_client: web::Data<RedisClient>, user_id: web::Path<u64>) -> impl Responder {
//             match redis_client.get_user(*user_id) {
//                 Ok(Some(user)) => HttpResponse::Ok().json(user),
//                 Ok(None) => HttpResponse::NotFound().json("User not found"),
//                 Err(_) => HttpResponse::InternalServerError().json("Error retrieving user"),
//             }
//         }
//     }

//     pub mod post_handlers {
//         use super::*;

//         pub async fn create_post(redis_client: web::Data<RedisClient>, post: web::Json<Post>) -> impl Responder {
//             match redis_client.save_post(&post.into_inner()) {
//                 Ok(_) => HttpResponse::Created().json("Post created"),
//                 Err(_) => HttpResponse::InternalServerError().json("Error creating post"),
//             }
//         }

//         pub async fn get_post(redis_client: web::Data<RedisClient>, post_id: web::Path<u64>) -> impl Responder {
//             match redis_client.get_post(*post_id) {
//                 Ok(Some(post)) => HttpResponse::Ok().json(post),
//                 Ok(None) => HttpResponse::NotFound().json("Post not found"),
//                 Err(_) => HttpResponse::InternalServerError().json("Error retrieving post"),
//             }
//         }
//     }
// }
