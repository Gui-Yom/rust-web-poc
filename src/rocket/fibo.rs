#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rust_web_poc::fibonacci::fibo;

#[get("/?<n>")]
fn index(n: u32) -> String {
    format!("{}", fibo(n))
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}


