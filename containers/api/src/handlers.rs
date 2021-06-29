use actix_web::{get, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ApiResponse {
    message: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json(ApiResponse {
        message: "Hello world from Rust!".to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_rt::test]
    async fn handlers_hello_ok() {
        let mut app = test::init_service(App::new().service(hello)).await;

        let payload = r#"{"message":"Hello world from Rust!"}"#.as_bytes();
        let res = test::TestRequest::get()
            .uri("/")
            .set_payload(payload)
            .send_request(&mut app)
            .await;

        assert!(res.status().is_success());

        let _result: ApiResponse = test::read_body_json(res).await;
    }
}
