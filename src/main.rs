#![allow(unused)]
//#![deny(warnings)]

use clap::Parser;
use warp::Filter;

#[derive(Parser, Debug, Clone)] // Clone?
pub struct Arguments {
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

    let api = filters::filters(args);

    //let echo_routes = api.with(warp::log("")); // Not working

    let health_check = warp::path!("health").map(|| {
        warp::http::Response::builder()
            .status(warp::http::StatusCode::OK)
            .body("\"status\": \"ok\"")
    });

    let routes = api.or(health_check);

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}

mod filters {
    use super::handlers;
    use super::Arguments;
    use warp::Filter;

    pub fn filters(
        args: Arguments,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        echo(args)
    }

    pub fn echo(
        args: Arguments,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!()
            .and(warp::get())
            .and(with_args(args))
            .and_then(handlers::echo)
    }

    fn with_args(
        args: Arguments,
    ) -> impl Filter<Extract = (Arguments,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || args.clone()) // Clone?
    }
}

mod handlers {
    use super::Arguments;
    use std::convert::Infallible; // What is this for?
    use warp::http::StatusCode;

    pub async fn echo(args: Arguments) -> Result<impl warp::Reply, Infallible> {
        Ok(warp::http::Response::builder()
            .status(args.response_status_code)
            .body(args.response_body))
    }
}
