use actix_web::{guard, web, App, HttpServer};
use crate::gql::{build_schema, graphiql_handler, graphql_handler};


mod gql;
mod dbs;
mod models;
mod service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let schema = build_schema().await;
    println!("Listening on: http://localhost:8081/");
    HttpServer::new(move || {
        App::new().app_data(web::Data::new(schema.clone()))
            .service(web::resource("/graphql").guard(guard::Post()).to(graphql_handler))
            .service(web::resource("/graphiql").guard(guard::Get()).to(graphiql_handler))
    })
        .bind("127.0.0.1:8082")?
        .run()
        .await
}
