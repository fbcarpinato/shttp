use std::{
    io::{Read, Result, Write},
    net::{TcpListener, TcpStream},
};

use crate::{http_status::HttpStatus, response::HttpResponse};

pub struct HttpServer {
    listener: TcpListener,
}

impl HttpServer {
    pub fn new(server_addr: &str) -> Result<Self> {
        let listener = TcpListener::bind(server_addr)?;

        Ok(HttpServer { listener })
    }
    pub fn start(&self) -> Result<()> {
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    self.handle_client(stream)?;
                }
                Err(e) => {
                    eprintln!("Failed to establish a connection: {}", e);
                }
            }
        }
        Ok(())
    }

    fn handle_client(&self, mut stream: TcpStream) -> Result<()> {
        let mut buffer = [0; 1024];

        stream.read(&mut buffer).unwrap();

        let request = String::from_utf8_lossy(&buffer[..]);

        println!("request: {}", request);

        let mut response = HttpResponse::html(HttpStatus::Ok, "<div>hello</div>".to_string());

        response.set_header("Custom", "test");

        stream.write(&response.as_bytes()).unwrap();

        Ok(())
    }
}
