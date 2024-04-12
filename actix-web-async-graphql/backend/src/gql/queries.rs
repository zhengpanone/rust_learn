use async_graphql::{FieldResult, Object, Context};
use sqlx::{MySqlPool, Row};
use crate::models::user::User;
use crate::service::user as userService;

// 定义一个 GraphQL 的查询对象
#[derive(Default)]
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }

    async fn hello(&self) -> FieldResult<String> {
        Ok("Hello,GraphQL!".to_string())
    }

    async fn get_user(&self, context: &Context<'_>, user_id: i32) -> FieldResult<String> {
        let pool = context.data::<MySqlPool>()?;
        let row = sqlx::query("SELECT * FROM user WHERE id = ?")
            .bind(user_id).fetch_one(pool).await?;
        let name: String = row.try_get("username")?;
        Ok(name)
    }

    async fn get_all_users(&self, context: &Context<'_>, user_id: i32) -> FieldResult<Vec<User>> {
        let pool = context.data::<MySqlPool>()?;
        let result = userService::all_users(pool, user_id).await?;
        Ok(result)
    }
}