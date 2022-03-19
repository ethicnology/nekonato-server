use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
#[macro_use]
extern crate dotenv_codegen;

async fn index(_req: HttpRequest) -> impl Responder {
    "Wesh"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file(dotenv!("SSL_KEY"), SslFiletype::PEM)
        .unwrap();
    builder
        .set_certificate_chain_file(dotenv!("SSL_CRT"))
        .unwrap();
    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind_openssl((dotenv!("IP"), dotenv!("PORT")), builder)?
        .run()
        .await
}
