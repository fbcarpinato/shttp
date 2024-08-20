use std::collections::HashMap;

use crate::http_status::HttpStatus;

pub struct HttpResponse {
    status: HttpStatus,
    headers: HashMap<String, String>,
    body: String
}

impl HttpResponse {
    pub fn html(status: HttpStatus, body: String) -> Self {
        let mut headers = HashMap::new();

        headers.insert("Content-Type".to_string(), "text/html".to_string());
        headers.insert("Content-Length".to_string(), body.len().to_string());

        HttpResponse { status, headers, body }
    }

    pub fn set_header(&mut self,header: &str, value: &str) {
        self.headers.insert(header.to_string(), value.to_string());
    }


    pub fn as_bytes(&self) -> Vec<u8> {
        let headers = self.headers.iter()
            .map(|(key, value)| format!("{}: {}", key, value))
            .collect::<Vec<String>>()
            .join("\n");

        let response = format!(
            "HTTP/1.1 {}\n{}\n\n{}",
            self.status,
            headers,
            self.body
        );

        println!("{}", response);

        response.into_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_header_new_header() {
        let mut response = HttpResponse::html(HttpStatus::Ok, "<div>hello</div>".to_string());

        response.set_header("X-Test-Header", "TestValue");

        let response_bytes = response.as_bytes();
        let response_string = String::from_utf8(response_bytes).expect("Response should be valid UTF-8");

        assert!(response_string.contains("HTTP/1.1 200 OK"));

        assert!(response_string.contains("X-Test-Header: TestValue"));

        assert!(response_string.contains("Content-Type: text/html"));

        assert!(response_string.contains("Content-Length: 16"));

        assert!(response_string.contains("<div>hello</div>"));
    }
}
