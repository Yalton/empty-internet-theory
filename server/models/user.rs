use serde::{Deserialize, Serialize};
use redis::Commands;
use crate::db::redis_client::RedisClient;
use crate::utils::error::ServiceError;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub id: String,
    pub displayname: String,
    pub username: String,
    pub pfp: String,
    pub bio: String,
    pub fwing: String,
    pub fwers: String,
    pub bot: bool,
}

impl User {
    pub fn new(id: String, displayname: String, username: String, pfp: String, bio: String, fwing: String, fwers: String, bot: bool) -> User {
        User {
            id,
            displayname,
            username,
            pfp,
            bio,
            fwing,
            fwers,
            bot,
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
