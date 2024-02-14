use std::fs;

use axum::{
    body::Body,
    extract::Request,
    middleware::{self, Next},
    response::Response,
    Router,
};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    print!("\x1B[2J\x1B[1;1H");
    print!("🌎 Iniciando servidor fake na porta \"8765\"...");
    let app = Router::new()
        .layer(middleware::from_fn(root_middleware))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_headers(Any)
                .allow_methods(Any),
        );
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8765").await.unwrap();

    println!("\t\t\tIniciado! ✅");

    axum::serve(listener, app).await.unwrap();
}

async fn root_middleware(request: Request, next: Next) -> Response {
    let method = request.method().to_string();
    let uri = request.uri().to_string();
    let uri = uri.split("?").collect::<Vec<&str>>()[0];

    let path = format!("paths/{}.json", &uri[1..]);

    if let Ok(content) = fs::read_to_string(&path) {
        println!("{: <7}  ✅ 200     {}", &method, uri);
        return Response::new(Body::from(content));
    }

    println!("{: <7}  ❌ 404     {}", &method, uri);

    next.run(request).await
}
