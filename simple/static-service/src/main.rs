use axum::{
    routing::get,
    Router,
};

#[tokio::main(flavor="current_thread")]
async fn main() {
    println!("starting server");

    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello From in Server" }));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
