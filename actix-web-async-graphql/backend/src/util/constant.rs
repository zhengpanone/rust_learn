use std::collections::HashMap;
use dotenv::dotenv;
use lazy_static::lazy_static;

lazy_static! {
 pub static ref CFG:HashMap<&'static str, String>={
        dotenv().ok();
        let mut map = HashMap::new();
        map.insert(
            "ADDRESS",
            dotenv::var("ADDRESS").expect("Expected ADDRESS to be set in env!"),
        );

        map.insert(
            "PORT",
            dotenv::var("PORT").expect("Expected port to be set in env!"),
        );

        map.insert(
            "GIQL_VER",
            dotenv::var("GIQL_VER").expect("Expected port to be set in env!"),
        );

        map.insert(
            "DATABASE_URL",
            dotenv::var("DATABASE_URL").expect("Expected DATABASE_URL to be set in env!"),
        );
        map
    };
}