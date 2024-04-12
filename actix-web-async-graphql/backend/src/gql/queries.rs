use async_graphql::Object;

// 定义一个 GraphQL 的查询对象
#[derive(Default)]
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}