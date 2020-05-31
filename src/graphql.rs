#![feature(decl_macro, proc_macro_hygiene)]

#[macro_use]
extern crate rocket;

use juniper::{
    EmptyMutation,
    RootNode, tests::{model::Database, schema::Query},
};
use rocket::{response::content, State};

type Schema = RootNode<'static, Query, EmptyMutation<Database>>;

#[get("/")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[get("/graphql?<request>")]
fn get_graphql_handler(
    context: State<Database>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

#[post("/graphql", data = "<request>")]
fn post_graphql_handler(
    context: State<Database>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

fn main() {
    rocket::ignite()
        .manage(Database::new())
        .manage(Schema::new(
            Query,
            EmptyMutation::<Database>::new(),
        ))
        .mount(
            "/",
            rocket::routes![graphiql, get_graphql_handler, post_graphql_handler],
        )
        .launch();
}