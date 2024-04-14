use async_graphql::{ErrorExtensions, Error};
use sqlx::{MySqlPool, query, query_as, query_scalar, Result};
use sqlx::mysql::MySqlQueryResult;
use crate::form::user::{UpdateUserForm, NewUserForm};
use crate::models::user::User;
use crate::util::constant::GlqResult;

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

pub async fn insert_user(pool: &MySqlPool, user: NewUserForm) -> Result<User, Error> {
    let count = get_user_by_email(pool, &user.email).await?;

    if count > 0 {
        return Err(Error::new("Email already exists").extend_with(|_, e| e.set("details", "EMAIL_EXISTS")));
    }

    // 获取有效的用户名，如果是 None 则使用默认值 ""
    // 获取有效的用户名，如果是 None 则使用默认值 ""
    println!("----------");
    let username = user.username;
    let email = user.email.unwrap_or_default();

    // 执行插入操作
    let result: MySqlQueryResult = query!("insert into user (username, email, cred) values(?,?,?)",
           username,
            email,
            user.cred.unwrap_or_default()
        ).execute(pool)
        .await.map_err(|err| {
        Error::new("SQLX exception")
            .extend_with(|_, e| e.set("details", err.to_string()))
    })?;

    if result.rows_affected()!=1{
        return Err(Error::new("Failed to insert user"));
    }
    // 处理插入结果
    Ok(
        User {
            id: result.last_insert_id() as u32,
            username,
            email: Some(email),
            cred: None,

        })
}

/// 根据用户邮箱查询用户信息
pub async fn get_user_by_email(pool: &MySqlPool, email: &Option<String>) -> GlqResult<i64> {
    let count: Option<i64> = query_scalar!("select count(1) from user where email = ?",email)
        .fetch_optional(pool).await?;
    // 如果 count 为 None，则返回 0，否则返回 count 的值
    Ok(count.unwrap_or(0))
}