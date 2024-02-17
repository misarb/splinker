use actix_web::{get, post, App, HttpRequest, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello Reg")
}

#[get("/test")]
async fn tesReq() -> impl Responder {
    HttpResponse::Ok().body("req responder test")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello).service(tesReq))
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
