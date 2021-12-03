use std::sync::Arc;
use std::sync::Mutex;

use actix_web::{get, post, put, web, HttpResponse};

use crate::service::{ChangeTableDTO, CreateTableDTO, TableService};

pub struct TableAppState {
    pub table_service: Arc<Mutex<TableService>>,
}

impl TableAppState {
    pub fn new() -> TableAppState {
        TableAppState {
            table_service: Arc::new(Mutex::new(TableService::new())),
        }
    }
}

#[post("")]
pub async fn create_table(
    data: web::Data<TableAppState>,
    body: web::Json<CreateTableDTO>,
) -> HttpResponse {
    let mut table_service = data.table_service.lock().unwrap();
    let table = table_service.create_table(body.into_inner()).unwrap();
    HttpResponse::Ok().json(table)
}

#[get("/{table_id}")]
pub async fn get_table(
    data: web::Data<TableAppState>,
    web::Path(table_id): web::Path<String>,
) -> HttpResponse {
    let table_service = data.table_service.lock().unwrap();
    match table_service.get_table(table_id) {
        Ok(table) => HttpResponse::Ok().json(table),
        Err(_) => HttpResponse::BadRequest().body("Table not found"),
    }
}

#[put("")]
pub async fn change_table(
    data: web::Data<TableAppState>,
    body: web::Json<ChangeTableDTO>,
) -> HttpResponse {
    let mut table_service = data.table_service.lock().unwrap();
    match table_service.change_table(body.into_inner()) {
        Ok(table) => {
            println!("{:?}", table);
            return HttpResponse::Ok().json(true);
        }
        Err(_) => HttpResponse::BadRequest().body("I wasn't possible change the table"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web::Bytes, App};

    #[actix_rt::test]
    async fn test_post_table() {
        let mut app = test::init_service(
            App::new().service(
                web::scope("/table")
                    .data(TableAppState {
                        table_service: Arc::new(Mutex::new(TableService::new())),
                    })
                    .service(create_table),
            ),
        )
        .await;
        let body = CreateTableDTO {
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
    async fn test_get_table_error_not_found() {
        let service = Arc::new(Mutex::new(TableService::new()));

        let mut app = test::init_service(
            App::new().service(
                web::scope("/table")
                    .data(TableAppState {
                        table_service: service.clone(),
                    })
                    .service(get_table),
            ),
        )
        .await;
        let req = test::TestRequest::get().uri("/table/test").to_request();
        let result = test::read_response(&mut app, req).await;
        assert_eq!(result, Bytes::from_static(b"Table not found"));
    }

    #[actix_rt::test]
    async fn test_get_table() {
        let service = Arc::new(Mutex::new(TableService::new()));

        let dto = CreateTableDTO {
            id: String::from("test"),
            size_y: 1,
            size_x: 1,
        };
        service.lock().unwrap().create_table(dto).unwrap();

        let mut app = test::init_service(
            App::new().service(
                web::scope("/table")
                    .data(TableAppState {
                        table_service: service.clone(),
                    })
                    .service(get_table),
            ),
        )
        .await;

        let req = test::TestRequest::get().uri("/table/test").to_request();
        let result = test::read_response(&mut app, req).await;
        assert_eq!(
            result,
            Bytes::from_static(b"{\"id\":\"test\",\"size_x\":1,\"size_y\":1,\"data\":[[\"\"]]}")
        );
    }

    #[actix_rt::test]
    async fn test_change_table_error() {
        let service = Arc::new(Mutex::new(TableService::new()));

        let dto = CreateTableDTO {
            id: String::from("test"),
            size_y: 1,
            size_x: 1,
        };
        service.lock().unwrap().create_table(dto).unwrap();

        let mut app = test::init_service(
            App::new().service(
                web::scope("/table")
                    .data(TableAppState {
                        table_service: service.clone(),
                    })
                    .service(change_table),
            ),
        )
        .await;

        let body = ChangeTableDTO {
            id: String::from("notfound"),
            x: 0,
            y: 0,
            value: String::from("new value"),
        };
        let req = test::TestRequest::put()
            .uri("/table")
            .set_json(&body)
            .to_request();
        let result = test::read_response(&mut app, req).await;
        assert_eq!(
            result,
            Bytes::from_static(b"I wasn't possible change the table")
        );
    }

    #[actix_rt::test]
    async fn test_change_table() {
        let service = Arc::new(Mutex::new(TableService::new()));

        let dto = CreateTableDTO {
            id: String::from("test"),
            size_y: 1,
            size_x: 1,
        };
        service.lock().unwrap().create_table(dto).unwrap();

        let mut app = test::init_service(
            App::new().service(
                web::scope("/table")
                    .data(TableAppState {
                        table_service: service.clone(),
                    })
                    .service(change_table),
            ),
        )
        .await;

        let body = ChangeTableDTO {
            id: String::from("test"),
            x: 0,
            y: 0,
            value: String::from("new value"),
        };
        let req = test::TestRequest::put()
            .uri("/table")
            .set_json(&body)
            .to_request();
        let result = test::read_response(&mut app, req).await;
        assert_eq!(result, Bytes::from_static(b"true"));
    }
}
