#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[get("/?<n>")]
fn index(n: u32) -> String {
    format!("{}", fibo(n))
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}

const GOLDEN_RATIO: f64 = 1.6180339887;

fn fibo(n: u32) -> u64 {
    (GOLDEN_RATIO.powi(n as i32) / (5.0_f64).sqrt() as f64 + 0.5).trunc() as u64
}
