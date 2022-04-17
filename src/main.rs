use warp::Filter;

#[macro_use(lazy_static)]
extern crate lazy_static;

mod handlers;
mod models;
mod error;
mod settings;

#[tokio::main]
async fn main() {
    // let root = warp::path::end().map(|| "Hello, world!");

    // Path Definitions
    let link = warp::path("auth")
        .and(warp::header("X-FORWARDED-Uri"))
        .map(|uri: String| format!("URI: {}", uri));

    let validate = warp::path("validate")
        .and(warp::path::param())
        .and_then(handlers::validate_jwt);

    let gen = warp::path("gen").map(|| format!("{}", handlers::create_jwt().expect("died here")));

    let routes = warp::get().and(link.or(gen).or(validate).recover(error::handle_rejection));

    warp::serve(routes)
        .run(([127, 0, 0, 1], settings::CONFIG.server.port))
        .await;
}
