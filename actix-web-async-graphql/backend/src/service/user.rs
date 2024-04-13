use async_graphql::{ErrorExtensions, Error};
use sqlx::{MySqlPool, query, query_as, Result};
use crate::form::user::{UpdateUserForm, UserForm};
use crate::models::user::User;

pub async fn all_users(pool: &MySqlPool, id: u32) -> Result<Vec<User>, Error> {
    let users = match query_as!(User,
        "select `id`, `username`, `email`,`cred` from user where id < ?",
        id)
        .fetch_all(pool)
        .await
    {
        Ok(users) => users,
        Err(err) => {
            // 如果查询失败，将 sqlx::Error 转换为 async_graphql::Error，并添加自定义错误信息
            let error_msg = format!("Failed to fetch users: {}", err);
            return Err(Error::new(error_msg).extend_with(|_, e| e.set("details", "Database error")));
        }
    };

    if users.is_empty() {
        return Err(Error::new("No records found").extend_with(|_, e| e.set("details", "No records")));
    }
    Ok(users)
}

/**
 * 根据ID 获取用户信息
 */
pub async fn fetch_user_by_id(pool: &MySqlPool, id: u32) -> Result<User> {
    let users = query_as!(User, "select `id`, `username`, `email`,`cred` from user where id = ?", id).fetch_one(pool).await?;
    Ok(users)
}

/**
 * 根据ID 删除用户
 */
pub async fn delete_user_by_id(pool: &MySqlPool, id: u32) -> Result<()> {
    query!("delete from user where id = ?",id).execute(pool).await?;
    Ok(())
}

pub async fn update_user(pool: &MySqlPool, user: UpdateUserForm) -> Result<()> {
    query!("update user set username = ?, email = ? where id = ?",user.username,user.email,user.id).execute(pool).await?;
    Ok(())
}

pub async fn insert_user(pool: &MySqlPool, user: UserForm) -> Result<()> {
    let _ = query!("insert into user (username, email, cred) values(?,?,?)",user.username,user.email,user.cred).execute(pool).await?;
    Ok(())
}