use axum::{response::Html, routing::get, Router};

async fn hello_handler() -> Html<&'static str>{
    Html("<h1>Hello, World!</h1>")
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello_handler));
    
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server running on http://localhost:3000");

    axum::serve(listener, app)
        .await
        .unwrap();

}


