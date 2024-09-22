use shttp::http_status::HttpStatus;
use shttp::response::HttpResponse;
use shttp::server::HttpServer;
use tokio;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new("127.0.0.1:8080")
        .await
        .expect("Failed to create the server");

    server.get("/", |_| {
        let response = HttpResponse::html(
            HttpStatus::Ok,
            "<div>this is the index route and not a default handler</div>".to_string(),
        );

        response
    });

    server.get("/test", |_| {
        let response = HttpResponse::html(
            HttpStatus::Ok,
            "<div>this is the test route</div>".to_string(),
        );

        response
    });

    let _ = server.start().await;

    Ok(())
}
