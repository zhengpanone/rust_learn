use async_graphql::{FieldResult, Object, Context};
use sqlx::MySqlPool;

use crate::form::user::{UpdateUserForm, UserForm};



#[derive(Default)]
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn insert_user(&self, context: &Context<'_>, user: UserForm) -> FieldResult<String> {
        let pool = context.data::<MySqlPool>()?;
        let _ = crate::service::user::insert_user(pool, user).await?;
        Ok("insert user success".to_string())
    }

    async fn update_user(&self, context: &Context<'_>, user: UpdateUserForm) -> FieldResult<String> {
        let pool = context.data::<MySqlPool>()?;
        let _ = crate::service::user::update_user(pool, user).await?;
        Ok("insert user success".to_string())
    }

    async fn delete_user_by_id(&self, context: &Context<'_>, user_id: u32) -> FieldResult<String> {
        let pool = context.data::<MySqlPool>()?;
        let _= crate::service::user::delete_user_by_id(pool, user_id).await?;
        Ok(format!("delete user {} success", user_id))
    }
}