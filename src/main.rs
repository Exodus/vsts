use vsts::{create_jwt, error, validate_jwt, CONFIG};
use warp::Filter;

#[tokio::main]
async fn main() {
    // let root = warp::path::end().map(|| "Hello, world!");

    // Path Definitions
    let link = warp::path("link")
        .and(warp::path::param())
        .map(|jwt: String| format!("JWT: {}", jwt));

    let validate = warp::path("validate")
        .and(warp::path::param())
        .and_then(validate_jwt);

    let gen = warp::path("gen").map(|| format!("{}", create_jwt().expect("died here")));

    let routes = warp::get().and(link.or(gen).or(validate).recover(error::handle_rejection));

    warp::serve(routes)
        .run(([127, 0, 0, 1], CONFIG.server.port))
        .await;
}
