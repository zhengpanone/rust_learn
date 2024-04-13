use async_graphql::{FieldResult, Object, Context};
use sqlx::MySqlPool;

use crate::form::user::UserForm;


#[derive(Default)]
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn insert_user(&self, context: &Context<'_>, user: UserForm) -> FieldResult<String> {

        let pool = context.data::<MySqlPool>()?;
        let result = crate::service::user::insert_user(pool, user).await?;
        Ok("insert user success".to_string())
    }


}