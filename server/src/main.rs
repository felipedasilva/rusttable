use std::sync::Mutex;

use actix_web::{post, web, App, HttpResponse, HttpServer, Result};

use table::{CreateTableBody, TableService};

struct AppState {
    table_service: Mutex<TableService>,
}

#[post("/table")]
async fn create_table(
    data: web::Data<AppState>,
    body: web::Json<CreateTableBody>,
) -> Result<HttpResponse> {
    let mut table_service = data.table_service.lock().unwrap();
    let table = table_service.create_table(body.into_inner()).unwrap();
    Ok(HttpResponse::Ok().json(table))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(AppState {
                table_service: Mutex::new(TableService::new()),
            })
            .service(create_table)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web::Bytes, App};

    #[actix_rt::test]
    async fn test_table_post() {
        let mut app = test::init_service(
            App::new()
                .data(AppState {
                    table_service: Mutex::new(TableService::new()),
                })
                .service(create_table),
        )
        .await;
        let body = CreateTableBody {
            id: String::from("test"),
            size_x: 1,
            size_y: 1,
        };
        let req = test::TestRequest::post()
            .uri("/table")
            .set_json(&body)
            .to_request();
        let result = test::read_response(&mut app, req).await;
        assert_eq!(
            result,
            Bytes::from_static(b"{\"id\":\"test\",\"size_x\":1,\"size_y\":1,\"data\":[[\"\"]]}")
        );
    }
}
