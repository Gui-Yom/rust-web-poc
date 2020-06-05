### rust-web-poc
A quick comparison of some rust web frameworks.
The goal here is to show rust can be viable option for backend development.

#### Fibonacci sequence
With rocket :

Run `cargo run --bin rocket_fibo --features=rocket` and visit `http://localhost:8000/?n=5` 
to show the 5th number of the fibonacci sequence.

With actix :

Run `cargo run --bin actix_fibo --features=actix` and visit `http://localhost:8888/?n=5` 
to show the 5th number of the fibonacci sequence.

#### GraphQL server

Run `cargo run --bin graphql --features=graphql` and
visit `https://localhost:8000/` to get the GraphQLi interface.
