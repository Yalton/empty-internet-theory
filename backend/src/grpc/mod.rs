pub mod service;

use tonic::{transport::Server, Request, Response, Status};

use service::{MyTwitter, MyTwitterServer};

pub mod my_twitter {
    tonic::include_proto!("my_twitter");
}

use my_twitter::{
    PostRequest, PostResponse, UserRequest, UserResponse,
};

use crate::db::redis_client::RedisClient;
use crate::config::Config;
use crate::models::{User, Post};

pub struct MyTwitterService {
    redis_client: RedisClient,
}

#[tonic::async_trait]
impl MyTwitter for MyTwitterService {
    async fn create_user(
        &self,
        request: Request<UserRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        let user_request = request.into_inner();
        let user = User::new(
            user_request.id,
            user_request.username,
            user_request.email,
            user_request.password,
        );

        user.save(&self.redis_client)
            .map_err(|e| Status::internal(e.to_string()))?;

        let response = UserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
        };

        Ok(Response::new(response))
    }

    async fn create_post(
        &self,
        request: Request<PostRequest>,
    ) -> Result<Response<PostResponse>, Status> {
        let post_request = request.into_inner();
        let post = Post::new(
            post_request.id,
            post_request.user_id,
            post_request.content,
        );

        self.redis_client.save_post(&post)
            .map_err(|e| Status::internal(e.to_string()))?;

        let response = PostResponse {
            id: post.id,
            user_id: post.user_id,
            content: post.content,
            timestamp: post.timestamp,
        };

        Ok(Response::new(response))
    }

    // Implement other gRPC methods as needed
}

pub async fn run_grpc_server(redis_client: RedisClient, config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let addr = format!("[::1]:{}", config.server_port).parse()?;
    let my_twitter_service = MyTwitterService { redis_client };

    Server::builder()
        .add_service(MyTwitterServer::new(my_twitter_service))
        .serve(addr)
        .await?;

    Ok(())
}
