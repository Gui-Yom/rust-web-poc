use actix_web::{App, HttpServer, web};
use serde::Deserialize;

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

const GOLDEN_RATIO: f64 = 1.6180339887;

fn fibo(n: u32) -> u64 {
    (GOLDEN_RATIO.powi(n as i32) / (5.0_f64).sqrt() as f64 + 0.5).trunc() as u64
}
