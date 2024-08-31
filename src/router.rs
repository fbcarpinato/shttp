use crate::{request::Request, response::HttpResponse};
use std::collections::HashMap;

pub struct Router {
    routes: HashMap<String, Box<dyn Fn(&Request) -> HttpResponse>>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            routes: HashMap::new(),
        }
    }

    pub fn get_route_handler_for_request(
        &self,
        request: &Request,
    ) -> Option<&Box<dyn Fn(&Request) -> HttpResponse>> {
        let path = request.path();
        self.routes.get(path)
    }

    pub fn get<F: Fn(&Request) -> HttpResponse + 'static>(&mut self, path: &str, handler: F) {
        self.routes.insert(path.to_string(), Box::new(handler));
    }
}
