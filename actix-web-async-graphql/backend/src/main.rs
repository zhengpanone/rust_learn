use actix_web::{guard, web, App, HttpServer};
use crate::gql::{build_schema, graphiql_handler, graphql_handler};
use crate::util::constant::CFG;


mod gql;
mod dbs;
mod models;
mod service;
mod form;
mod util;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let schema = build_schema().await;
    println!("Listening on: http://localhost:8081/ \n GraphQL UI:http://{}:{}/{}",
             CFG.get("ADDRESS").unwrap(),
             CFG.get("PORT").unwrap(),
             CFG.get("GIQL_VER").unwrap()
    );
    HttpServer::new(move || {
        App::new().app_data(web::Data::new(schema.clone()))
            .service(web::resource(CFG.get("GIQL_VER").unwrap()).guard(guard::Post()).to(graphql_handler))
            .service(web::resource(CFG.get("GIQL_VER").unwrap()).guard(guard::Get()).to(graphiql_handler))
    })
        .bind(format!("{}:{}",
                      CFG.get("ADDRESS").unwrap(),
                      CFG.get("PORT").unwrap()
        ))?
        .run()
        .await
}
