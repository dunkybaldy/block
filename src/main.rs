use actix_web::{HttpServer, App, web};
use block::controller::user_controller;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(user_controller::hello)
            .service(user_controller::echo)
            .route("/hey", web::get().to(user_controller::manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
