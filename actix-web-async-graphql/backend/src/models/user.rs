use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, Row, Result};
use sqlx::mysql::MySqlRow;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub email: Option<String>,
    pub cred: Option<String>, // 使用 Option<String> 来处理可能的空值
}

// User 结构体中定义的字段类型为 String，但结构体实现中返回为 &str，
// 这是因为 Rust 中 String 未有默认实现 copy trait。如果您希望结构体实现中返回 String，可以通过 clone() 方法实现：
#[async_graphql::Object]
impl User {
    pub async fn id(&self) -> u32 {
        self.id
    }
    pub async fn username(&self) -> &str {
        self.username.as_str()
    }
    pub async fn email(&self) -> &str {
        match &self.email {
            Some(s) => s.as_str(), // 如果 email 字段存在，则返回其字符串引用
            None => "", // 如果 email 字段为 None，则返回空字符串
        }
    }

    pub async fn cred(&self) -> &str {
        match &self.cred {
            Some(s) => s.as_str(),
            None => "",
        }
    }
}

/**
 * 为 User 结构体实现了 FromRow trait，该 trait 的实现用于从 MySqlRow 转换为 User 对象
 */
impl<'r> FromRow<'r, MySqlRow> for User {
    fn from_row(row: &'r MySqlRow) -> Result<Self, Error> {
        let username = row.try_get("username")?;
        let email: Option<String> = row.try_get::<Option<String>, _>("email").unwrap_or_else(|_| None); // 如果成功获取到值，则将其赋给 email 字段，否则将 email 字段设置为 None
        let cred = row.try_get::<Option<String>, _>("cred").unwrap_or_else(|_| None);
        Ok(User {
            id: row.try_get("id")?,
            username,
            email,
            cred,
        })
    }
}