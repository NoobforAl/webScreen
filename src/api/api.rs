use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[path = "../core/webscreen.rs"]
mod web_screen;

#[derive(Serialize, Deserialize, Debug)]
struct Info {
    url: String,

    #[serde(default = "default_quality")]
    quality: u32,

    #[serde(default = "default_format")]
    format: String,

    #[serde(default = "default_timeout")]
    timeout: u64,
}

fn default_timeout() -> u64 {
    10
}

fn default_quality() -> u32 {
    100
}

fn default_format() -> String {
    String::from("png")
}

#[get("/")]
async fn screenshot(q: web::Query<Info>) -> impl Responder {
    let res = web_screen::web_screen(&q.url, q.timeout, &q.format, q.quality);

    let content_type = match q.format.as_str() {
        "png" => "image/png",
        "jpeg" => "image/jpeg",
        _ => "image/png",
    };

    match res {
        Ok(data) => HttpResponse::Ok().content_type(content_type).body(data),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {}", err)),
    }
}

#[actix_web::main]
pub async fn run(port: u16) -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(screenshot))
        .bind(("0.0.0.0", port))?
        .run()
        .await
}
