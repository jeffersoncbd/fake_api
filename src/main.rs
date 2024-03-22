use std::fs;

use axum::{
    body::Body,
    extract::Request,
    middleware::{self, Next},
    response::Response,
    Router,
};
// use http::StatusCode;
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

fn find_file(path: String) -> Option<Response> {
    if let Ok(content) = fs::read_to_string(format!("{path}.json")) {
        let response = Response::new(Body::from(content));
        // *response.status_mut() = StatusCode::BAD_REQUEST;
        return Some(response);
    }
    None
}

async fn root_middleware(request: Request, next: Next) -> Response {
    let method = request.method().to_string();
    let full_uri = request.uri().to_string();
    let parts: Vec<&str> = full_uri.split("?").collect::<Vec<&str>>();
    let uri = parts[0];

    let mut query: Option<Vec<(&str, &str)>> = None;

    if parts.len() > 1 {
        query = Some(
            parts[1]
                .split("&")
                .collect::<Vec<&str>>()
                .iter()
                .map(|item| {
                    let parts = item.split("=").collect::<Vec<&str>>();
                    (parts[0], parts[1])
                })
                .collect::<Vec<(&str, &str)>>(),
        );
    };

    let mut path = format!("paths/{}", &uri[1..]);

    if let Some(response) = find_file(path.clone()) {
        println!("{: <7}  ✅ 200     {}", &method, uri);
        return response;
    };

    if let Some(query) = query {
        for item in query.iter() {
            path.push_str("/");
            path.push_str(item.0);
            if let Some(response) = find_file(path.clone()) {
                println!("{: <7}  ✅ 200     {}", &method, uri);
                return response;
            };

            path.push_str("/");
            path.push_str(item.1);
            if let Some(response) = find_file(path.clone()) {
                println!("{: <7}  ✅ 200     {}", &method, uri);
                return response;
            };
        }
    }

    println!("{: <7}  ❌ 404     {}", &method, full_uri);

    next.run(request).await
}
