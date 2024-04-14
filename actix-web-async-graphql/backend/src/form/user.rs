use async_graphql::{ComplexObject, InputObject, SimpleObject};
use serde::{Deserialize, Serialize};

#[derive(Debug, InputObject, Serialize, Deserialize, Clone, SimpleObject)]
#[graphql(complex)]
pub struct NewUserForm {
    #[graphql(skip)]
    pub id: u32,
    pub username: String,
    pub email: Option<String>,
    #[graphql(skip)]
    pub cred: Option<String>,
}

#[ComplexObject]
impl NewUserForm {
    pub async fn from(&self) -> String {
        let mut from = String::new();
        from.push_str(&self.username);
        from.push_str("<");
        from.push_str(&self.email.to_owned().unwrap());
        from.push_str(">");
        from
    }
}


#[derive(Debug, InputObject)]
pub struct UpdateUserForm {
    pub id: u32,
    pub username: String,
    pub email: Option<String>,
    pub cred: Option<String>,
}

