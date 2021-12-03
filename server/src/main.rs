mod app_table;

use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use env_logger::Env;

use app_table::{TableAppState, change_table, create_table, get_table};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    HttpServer::new(|| {
        App::new()
            .data(TableAppState::new())
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(get_table)
            .service(create_table)
            .service(change_table)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
