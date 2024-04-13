use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, Row, Result};
use sqlx::mysql::MySqlRow;
/**
 * SimpleObject 省去满篇的 getter、setter
 */
#[derive(Serialize, Deserialize, Debug, Clone, async_graphql::SimpleObject)]
#[graphql(complex)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub email: Option<String>,
    pub cred: Option<String>, // 使用 Option<String> 来处理可能的空值
}



#[async_graphql::ComplexObject]
impl User{
  pub async fn from(&self)->String{
      let mut from = String::new();
      from.push_str(&self.username);
      from.push_str("<");
      from.push_str(&self.email.to_owned().unwrap());
      from.push_str(">");
      from
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