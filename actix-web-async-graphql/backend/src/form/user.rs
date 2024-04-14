use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};

#[derive(Debug, InputObject, Serialize, Deserialize, Clone, SimpleObject)]
pub struct NewUserForm {
    #[graphql(skip)]
    pub id: u32,
    pub username: String,
    pub email: Option<String>,
    #[graphql(skip)]
    pub cred: Option<String>,
}


#[derive(Debug, InputObject)]
pub struct UpdateUserForm {
    pub id: u32,
    pub username: String,
    pub email: Option<String>,
    pub cred: Option<String>,
}

