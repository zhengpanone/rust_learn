use sqlx::{Pool, mysql::MySqlPoolOptions, MySql};

use crate::util::constant::CFG;

// 获取 MySql 数据池后，可以将其增加到：
// 1. 作为 async-graphql 的全局数据；
// 2. 作为 actix-web 的应用程序数据，可进行原子操作；；
// 3. 使用 lazy-static.rs
pub async fn establish_connection() -> Pool<MySql> {
    // 从环境变量中读取数据库连接信息

    let database_url = CFG.get("DATABASE_URL").unwrap();
    let pool = match MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await {
        Ok(pool) => pool,
        Err(err) => {
            eprintln!("Failed to connect to database: {}", err);
            std::process::exit(1); // 终止程序并返回错误码
        }
    };
    pool
}