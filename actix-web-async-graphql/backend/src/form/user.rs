use async_graphql::{InputObject, Object};

#[derive(Debug, InputObject)]
pub struct UserForm {
    pub username: Option<String>,
    pub email: Option<String>,
    pub cred: Option<String>,
}

#[Object]
impl UserForm {
    pub async fn username(&self) -> &str {
        match &self.username {
            Some(s) => s.as_str(),
            None => "",
        }
    }

    pub async fn email(&self) -> &str {
        match &self.email {
            Some(s) => s.as_str(), // 如果 email 字段存在，则返回其字符串引用
            None => "", // 如果 email 字段为 None，则返回空字符串
        }
    }

    pub async fn cred(&self) -> &str {
        match &self.cred {
            Some(s) => s.as_str(), // 如果 email 字段存在，则返回其字符串引用
            None => "", // 如果 email 字段为 None，则返回空字符串
        }
    }
}