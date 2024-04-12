use sqlx::{Pool,  mysql::MySqlPoolOptions, MySql};
use dotenv;
// 获取 MySql 数据池后，可以将其增加到：
// 1. 作为 async-graphql 的全局数据；
// 2. 作为 actix-web 的应用程序数据，可进行原子操作；；
// 3. 使用 lazy-static.rs
pub async fn establish_connection() -> Pool<MySql> {
    // 从环境变量中读取数据库连接信息
    dotenv::dotenv().ok();
    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool.");
    pool
}