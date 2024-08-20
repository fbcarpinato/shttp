use std::{
    fmt::Result,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use http_status::HttpStatus;
use response::HttpResponse;

mod http_status;
mod response;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream).unwrap();
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream) -> Result {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);

    println!("request: {}", request);

    let mut response = HttpResponse::html(HttpStatus::Ok, "<div>hello</div>".to_string());

    response.set_header("Custom", "test");

    stream.write(&response.as_bytes()).unwrap();

    Ok(())
}
