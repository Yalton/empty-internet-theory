use tonic::{Request, Response, Status};

use crate::db::redis_client::RedisClient;
use crate::models::{Post, User};
use crate::grpc::my_twitter::{
    self, MyTwitter, PostRequest, PostResponse, UserRequest, UserResponse,
};

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

    async fn get_user(
        &self,
        request: Request<UserRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        let user_request = request.into_inner();
        let user = User::find_by_id(&user_request.id, &self.redis_client)
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

        // Assuming there is a method `save` in Post similar to User's `save`
        post.save(&self.redis_client)
            .map_err(|e| Status::internal(e.to_string()))?;

        let response = PostResponse {
            id: post.id,
            user_id: post.user_id,
            content: post.content,
            timestamp: post.timestamp,
        };

        Ok(Response::new(response))
    }

    async fn get_post(
        &self,
        request: Request<PostRequest>,
    ) -> Result<Response<PostResponse>, Status> {
        let post_request = request.into_inner();
        // Assuming there is a method `find_by_id` in Post similar to User's `find_by_id`
        let post = Post::find_by_id(post_request.id, &self.redis_client)
            .map_err(|e| Status::internal(e.to_string()))?;

        let response = PostResponse {
            id: post.id,
            user_id: post.user_id,
            content: post.content,
            timestamp: post.timestamp,
        };

        Ok(Response::new(response))
    }

    // Add additional gRPC methods as needed for the functionality of your Twitter clone
}

// You will need to implement additional methods for the gRPC service as required by your application.
// This may include methods for updating and deleting users and posts, listing all posts for a user,
// handling followers/following, etc.

// Don't forget to add the necessary routes to your gRPC server in `src/grpc/mod.rs`.
