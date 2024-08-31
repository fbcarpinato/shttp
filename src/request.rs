use std::collections::HashMap;

pub struct Request {
    method: String,
    path: String,
    version: String,
    headers: HashMap<String, String>,
    body: Option<String>,
}

impl Request {
    pub fn from_buffer(buffer: &[u8; 1024]) -> Result<Self, String> {
        let request_str = String::from_utf8_lossy(buffer);

        let mut lines = request_str.lines();

        let request_line = lines
            .next()
            .ok_or("Invalid request: Missing request line")?;

        let mut request_parts = request_line.split_whitespace();

        let method = request_parts
            .next()
            .ok_or("Invalid request: Missing method")?
            .to_string();
        let path = request_parts
            .next()
            .ok_or("Invalid request: Missing path")?
            .to_string();
        let version = request_parts
            .next()
            .ok_or("Invalid request: Missing version")?
            .to_string();

        let mut headers = HashMap::new();

        for line in lines.by_ref() {
            if line.is_empty() {
                break; // Empty line signifies the end of headers
            }
            if let Some((key, value)) = line.split_once(": ") {
                headers.insert(key.to_string(), value.to_string());
            }
        }

        let body = if let Some(content_length) = headers.get("Content-Length") {
            let content_length = content_length
                .parse::<usize>()
                .map_err(|_| "Invalid Content-Length")?;
            let body: String = lines.collect::<Vec<&str>>().join("\n");
            if body.len() >= content_length {
                Some(body)
            } else {
                None
            }
        } else {
            None
        };

        Ok(Request {
            method,
            path,
            version,
            headers,
            body,
        })
    }

    pub fn method(&self) -> &str {
        &self.method
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn version(&self) -> &str {
        &self.version
    }
}
