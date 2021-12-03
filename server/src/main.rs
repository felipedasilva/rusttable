use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use env_logger::Env;
use std::sync::Arc;
use std::sync::Mutex;

use table::application::{change_table, create_table, get_table, TableAppState};
use table::service::TableService;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    // In order to share tables data
    // because each thread create it's own state
    let table_service = Arc::new(Mutex::new(TableService::new()));
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(
                web::scope("/table")
                    .data(TableAppState {
                        table_service: table_service.clone(),
                    })
                    .service(create_table)
                    .service(change_table)
                    .service(get_table),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
