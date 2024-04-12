pub mod mutations;
pub mod queries;


use actix_web::{HttpResponse, web, Result};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql::http::{GraphQLPlaygroundConfig, playground_source};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

use crate::dbs::mysql::establish_connection;

use crate::gql::queries::QueryRoot;

// 定义Graphql的Schema

// `ActixSchema` 类型定义，项目中可以统一放置在一个共用文件中。
// 但和 `actix-web` 和 `tide` 框架不同，无需放入应用程序`状态（State）`
// 所以此 `Schema` 类型仅是为了代码清晰易读，使用位置并不多，我们直接和构建函数一起定义。
// 或者，不做此类型定义，直接作为构建函数的返回类型。
type ActixSchema = Schema<
    QueryRoot,
    EmptyMutation,
    EmptySubscription
>;

// 创建 GraphQL schema
pub async fn build_schema() -> ActixSchema {
    let pool= establish_connection().await;
    // 构建 `Schema`
    // The root object for the query and Mutatio, and use EmptySubscription.
    // Add global sql datasource  in the schema object.
    Schema::build(QueryRoot::default(), EmptyMutation, EmptySubscription).data(pool).finish()
}

// 定义 GraphQL HTTP 服务处理函数
pub async fn graphql_handler(schema: web::Data<ActixSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}


pub async fn graphiql_handler() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(
        playground_source(
            GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/graphql"),
        ),
    ))
}
