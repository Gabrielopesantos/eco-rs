#![allow(unused)]

use clap::Parser;
use warp::Filter;

#[derive(Parser, Debug)]
struct Arguments {
    #[arg(long, default_value_t = (":8081").to_string())]
    listen: String,

    #[arg(long = "response_status", default_value_t = 200)]
    response_status_code: u16,

    #[arg(long = "response_body", default_value_t = ("Hello, World!").to_string())]
    response_body: String,
}

#[tokio::main] // main isn't allowed to be async without this
async fn main() {
    // Parse arguments
    let args = Arguments::parse();

    let health = warp::path!("health").map(|| "{\"status\": \"ok\"}");
    let echo = warp::path!().map(move || {
        args.response_body.clone()
    });

    let routes = echo.or(health);

    warp::serve(routes).run(([127, 0, 0, 1], 8081)).await;
}
