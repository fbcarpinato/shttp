use std::{
    io::{Read, Result, Write},
    net::{TcpListener, TcpStream},
};
use crate::{http_status::HttpStatus, request::Request, response::HttpResponse, router::Router};

pub struct HttpServer {
    listener: TcpListener,
    router: Router,
}

impl HttpServer {
    pub fn new(server_addr: &str) -> Result<Self> {
        let listener = TcpListener::bind(server_addr)?;

        let router = Router::new();

        Ok(HttpServer { listener, router })
    }
    pub fn start(&self) -> Result<()> {
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    self.handle_client(stream);
                }
                Err(e) => {
                    eprintln!("Failed to establish a connection: {}", e);
                }
            }
        }
        Ok(())
    }

    fn handle_client(&self, mut stream: TcpStream) {
        let mut buffer = [0; 1024];

        if let Err(e) = stream.read(&mut buffer) {
            eprintln!("Failed to read from stream: {}", e);
            return;
        }

        let parsed_request = match Request::from_buffer(&buffer) {
            Ok(parsed) => parsed,
            Err(e) => {
                eprintln!("Error parsing the request: {}", e);
                return;
            }
        };

        println!("Received request with method: {}", parsed_request.method());
        println!("Received request for path: {}", parsed_request.path());

        let mut response = HttpResponse::html(HttpStatus::Ok, "<div>hello</div>".to_string());

        response.set_header("custom", "test");

        if let Err(e) = stream.write(&response.as_bytes()) {
            eprintln!("Failed to write response to stream: {}", e);
            return;
        }
    }

    pub fn get(&mut self, path: &str, handler: fn()) {
        self.router.get(path, handler)
    }
}
