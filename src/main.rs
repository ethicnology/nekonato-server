use actix_web::{get, post, App, HttpRequest, HttpResponse, HttpServer, Responder};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use reqwest::*;
use serde_json::{from_str, json, Value};
use std::env;

#[get("/")]
async fn index(body: HttpRequest) -> impl Responder {
    println!("{:?}", body);
    let a = json!({
        "jsonrpc": "1.0",
        "id": "test",
        "method": "getnetworkinfo",
        "params": [],
    });
    HttpResponse::Ok().body(bridge(a).await.unwrap())
}

#[post("/")]
async fn echo(body: String) -> impl Responder {
    println!("{:?}", body);
    let a = bridge(from_str(&body).unwrap()).await.unwrap();
    HttpResponse::Ok().body(a)
}

async fn bridge(body: Value) -> Result<String> {
    let username = env::var("RPC_USER").unwrap();
    let password = env::var("RPC_PASS").unwrap();
    let client = reqwest::Client::new();
    let response = client
        .post(env::var("RPC_HOST").unwrap())
        .basic_auth(username, Some(password))
        .json(&body)
        .send()
        .await
        .unwrap();
    let text = response.text().await.unwrap();
    println!("{:?}", text);
    Ok(text)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!(
        "https://{} {} {}",
        env::var("HOST").unwrap(),
        env::var("SSL_KEY").unwrap(),
        env::var("SSL_CRT").unwrap()
    );
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file(env::var("SSL_KEY").unwrap(), SslFiletype::PEM)
        .unwrap();
    builder
        .set_certificate_chain_file(env::var("SSL_CRT").unwrap())
        .unwrap();
    HttpServer::new(|| App::new().service(index).service(echo))
        .bind_openssl(env::var("HOST").unwrap(), builder)?
        .run()
        .await
}
