use crate::{
    http_method::HttpMethod, http_status::HttpStatus, request::Request, response::HttpResponse,
};
use std::collections::HashMap;

pub struct Router {
    routes: HashMap<(HttpMethod, String), Box<dyn Fn(Request) -> HttpResponse + Send>>,
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
    ) -> Option<&Box<dyn Fn(Request) -> HttpResponse + Send>> {
        let path = request.path();
        let method = request.method();

        self.routes.get(&(method.clone(), path.to_string()))
    }

    pub fn default_handler(&self) -> Box<dyn Fn(Request) -> HttpResponse + Send> {
        Box::new(|_request: Request| {
            HttpResponse::html(
                HttpStatus::NotFound,
                "<span>Default 404 Not Found</span>".to_string(),
            )
        })
    }

    pub fn add_route<F>(&mut self, matcher: (HttpMethod, &str), handler: F)
    where
        F: Fn(Request) -> HttpResponse + Send + 'static,
    {
        self.routes
            .insert((matcher.0, matcher.1.to_string()), Box::new(handler));
    }
}
