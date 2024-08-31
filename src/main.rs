use http_status::HttpStatus;
use response::HttpResponse;
use server::HttpServer;

mod http_method;
mod http_status;
mod request;
mod response;
mod router;
mod server;

fn main() -> std::io::Result<()> {
    let mut server = HttpServer::new("127.0.0.1:8080")?;

    server.get("/", |_| {
        let response = HttpResponse::html(
            HttpStatus::Ok,
            "<div>this is the index route and not a default handler</div>".to_string(),
        );

        response
    });

    server.get("/test", |_| {
        let response =
            HttpResponse::html(HttpStatus::Ok, "<div>this is the test route".to_string());

        response
    });

    server.start()
}
