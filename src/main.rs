use server::HttpServer;

mod http_status;
mod response;
mod server;

fn main() -> std::io::Result<()> {
    let server = HttpServer::new("127.0.0.1:8080")?;

    server.start()
}
