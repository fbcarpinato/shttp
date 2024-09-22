use crate::{
    http_method::HttpMethod, http_status::HttpStatus, request::Request, response::HttpResponse,
    router::Router,
};
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::net::TcpStream;

pub struct HttpServer {
    listener: TcpListener,
    router: Arc<Mutex<Router>>,
}

impl HttpServer {
    pub async fn new(server_addr: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(server_addr).await?;
        let router = Arc::new(Mutex::new(Router::new()));

        Ok(HttpServer { listener, router })
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            let (stream, _) = self.listener.accept().await?;
            let router = Arc::clone(&self.router);

            tokio::spawn(async move {
                if let Err(e) = handle_client(stream, router).await {
                    eprintln!("Error handling client: {}", e);
                }
            });
        }
    }

    pub fn get<F>(&self, path: &str, handler: F)
    where
        F: Fn(Request) -> HttpResponse + Send + 'static,
    {
        let mut router = self.router.lock().unwrap();
        router.add_route((HttpMethod::GET, path), handler);
    }

    pub fn post<F>(&self, path: &str, handler: F)
    where
        F: Fn(Request) -> HttpResponse + Send + 'static,
    {
        let mut router = self.router.lock().unwrap();
        router.add_route((HttpMethod::POST, path), handler);
    }

    pub fn put<F>(&self, path: &str, handler: F)
    where
        F: Fn(Request) -> HttpResponse + Send + 'static,
    {
        let mut router = self.router.lock().unwrap();
        router.add_route((HttpMethod::PUT, path), handler);
    }

    pub fn patch<F>(&self, path: &str, handler: F)
    where
        F: Fn(Request) -> HttpResponse + Send + 'static,
    {
        let mut router = self.router.lock().unwrap();
        router.add_route((HttpMethod::PATCH, path), handler);
    }

    pub fn delete<F>(&self, path: &str, handler: F)
    where
        F: Fn(Request) -> HttpResponse + Send + 'static,
    {
        let mut router = self.router.lock().unwrap();
        router.add_route((HttpMethod::DELETE, path), handler);
    }
}

async fn handle_client(
    mut stream: TcpStream,
    router: Arc<Mutex<Router>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).await?;

    let parsed_request = match Request::from_buffer(&buffer) {
        Ok(parsed) => parsed,
        Err(e) => {
            eprintln!("Error parsing the request: {}", e);
            let response = HttpResponse::html(
                HttpStatus::BadRequest,
                "<span>Bad request!</span>".to_string(),
            );
            stream.write_all(&response.as_bytes()).await?;
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Error parsing the request",
            )));
        }
    };

    let response: HttpResponse = {
        let router = router.lock().unwrap();

        match router.get_route_handler_for_request(&parsed_request) {
            Some(handler) => handler(parsed_request),
            None => (router.default_handler())(parsed_request),
        }
    };

    stream.write_all(&response.as_bytes()).await?;

    Ok(())
}
