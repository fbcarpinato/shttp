use crate::{http_method::HttpMethod, request::Request, response::HttpResponse};
use std::collections::HashMap;

pub struct Router {
    routes: HashMap<(HttpMethod, String), Box<dyn Fn(Request) -> HttpResponse>>,
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
    ) -> Option<&Box<dyn Fn(Request) -> HttpResponse>> {
        let path = request.path();
        let method = request.method();

        self.routes.get(&(method.clone(), path.to_string()))
    }

    pub fn add_route<F: Fn(Request) -> HttpResponse + 'static>(&mut self, matcher: (HttpMethod, &str), handler: F) {
        self.routes
            .insert((matcher.0, matcher.1.to_string()), Box::new(handler));
    }
}
