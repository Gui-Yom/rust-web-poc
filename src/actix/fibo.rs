use actix_web::{App, HttpServer, web};
use serde::Deserialize;
use rust_web_poc::fibonacci::fibo;

#[derive(Deserialize)]
struct FiboReq {
    n: u32
}

async fn index(req: web::Query<FiboReq>) -> String {
    format!("{}", fibo(req.n))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route("/", web::get().to(index))
    }).bind("127.0.0.1:8088")?
        .run()
        .await
}
