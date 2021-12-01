use std::sync::Mutex;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Result};

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

#[get("/table/{table_id}")]
async fn get_table(
    data: web::Data<AppState>,
    web::Path(table_id): web::Path<String>,
) -> HttpResponse {
    let table_service = data.table_service.lock().unwrap();
    match table_service.get_table(table_id) {
        Ok(table) => HttpResponse::Ok().json(table),
        Err(_) => HttpResponse::BadRequest().body("Table not found"),
    }
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

    #[actix_rt::test]
    async fn test_table_get_error_not_found() {
        let service = Mutex::new(TableService::new());

        let mut app = test::init_service(
            App::new()
                .data(AppState {
                    table_service: service,
                })
                .service(get_table),
        )
        .await;
        let req = test::TestRequest::get().uri("/table/test").to_request();
        let result = test::read_response(&mut app, req).await;
        assert_eq!(result, Bytes::from_static(b"Table not found"));
    }

    #[actix_rt::test]
    async fn test_table_get() {
        let service = Mutex::new(TableService::new());

        let dto = CreateTableBody {
            id: String::from("test"),
            size_y: 1,
            size_x: 1,
        };
        service.lock().unwrap().create_table(dto).unwrap();

        let mut app = test::init_service(
            App::new()
                .data(AppState {
                    table_service: service,
                })
                .service(get_table),
        )
        .await;
        let req = test::TestRequest::get().uri("/table/test").to_request();
        let result = test::read_response(&mut app, req).await;
        assert_eq!(
            result,
            Bytes::from_static(b"{\"id\":\"test\",\"size_x\":1,\"size_y\":1,\"data\":[[\"\"]]}")
        );
    }
}
