use warp::Filter;

#[macro_use(lazy_static)]
extern crate lazy_static;

mod handlers;
mod models;
mod error;
mod settings;

#[tokio::main]
async fn main() {
    // Path Definitions
    let auth = warp::path("auth")
        .and(warp::header::<warp::http::Uri>("X-FORWARDED-Uri"))
        .and_then(handlers::auth);

    let gen = warp::path("gen").map(|| format!("{}", handlers::create_jwt().expect("Expecting Create JWT Token")));
    
    let validate = warp::path("validate")
        .and(warp::path::param())
        .and_then(handlers::validate_jwt);

    let healthcheck = warp::path("healthz")
            .map(|| format!("healthy!"));

    let routes = warp::get().and(auth.or(gen).or(validate).or(healthcheck).recover(error::handle_rejection));

    warp::serve(routes)
        .run(([0, 0, 0, 0], settings::CONFIG.server.port))
        .await;
}
