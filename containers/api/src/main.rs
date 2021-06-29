use actix_web::{middleware, App, HttpServer};
mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info,actix_web=debug");

    HttpServer::new(|| {
        App::new()
            .service(handlers::hello)
            .wrap(middleware::Logger::default())
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
