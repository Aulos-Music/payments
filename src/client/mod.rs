use axum::{Router, routing::MethodRouter};

pub struct Client {
    pub app: Router,
}

impl Client {
    pub fn new() -> Self {
        Client { app: Router::new() }
    }

    pub fn setup_get_route(mut self, name: &str, route: MethodRouter) -> Self {
        self.app = self.app.route(&format!("/{name}"), route);
        self
    }
}

impl Default for Client {
    fn default() -> Self {
        Client::new()
    }
}
