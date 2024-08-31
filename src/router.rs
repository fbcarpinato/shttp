use std::collections::HashMap;

pub struct Router {
    routes: HashMap<String, fn()>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            routes: HashMap::new(),
        }
    }

    pub fn get(&mut self, path: &str, handler: fn()) {
        self.routes.insert(path.to_string(), handler);
    }
}
