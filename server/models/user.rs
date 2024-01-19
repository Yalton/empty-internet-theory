use serde::{Deserialize, Serialize};
use redis::Commands;
use crate::db::redis_client::RedisClient;
use crate::utils::error::ServiceError;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    // You might want to store hashed passwords instead of plain text
    pub password: String,
}

impl User {
    pub fn new(id: String, username: String, email: String, password: String) -> User {
        User {
            id,
            username,
            email,
            password,
        }
    }

    pub fn save(&self, client: &RedisClient) -> Result<(), ServiceError> {
        let mut con = client.get_connection()?;
        let key = format!("user:{}", self.id);
        let value = serde_json::to_string(self)?;

        con.set::<String, String, String>(key, value)
            .map_err(|err| ServiceError::from(err))
    }

    pub fn find_by_id(id: &str, client: &RedisClient) -> Result<User, ServiceError> {
        let mut con = client.get_connection()?;
        let key = format!("user:{}", id);

        let user_data: String = con.get(key).map_err(|err| ServiceError::from(err))?;
        let user: User = serde_json::from_str(&user_data)?;

        Ok(user)
    }

    pub fn find_by_username(username: &str, client: &RedisClient) -> Result<User, ServiceError> {
        let mut con = client.get_connection()?;
        let users: Vec<String> = con.smembers("users")?;

        for user_id in users {
            let key = format!("user:{}", user_id);
            let user_data: String = con.get(&key)?;

            let user: User = serde_json::from_str(&user_data)?;
            if user.username == username {
                return Ok(user);
            }
        }

        Err(ServiceError::NotFound)
    }
}
