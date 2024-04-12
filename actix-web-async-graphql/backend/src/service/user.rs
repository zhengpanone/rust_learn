use sqlx::{MySqlPool, query_as,Result};
use crate::models::user::User;

pub async fn all_users(pool: &MySqlPool,id:i32) -> Result<Vec<User>> {
    let users = query_as!(User, "select `id`, `username`, `email`,`cred` from user where id < ?", id).fetch_all(pool).await?;
    Ok(users)
}