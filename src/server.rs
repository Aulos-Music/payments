use axum::{Router, routing::MethodRouter};

pub struct Server {
    pub app: Router,
}

impl Server {
    pub fn new() -> Self {
        Server { app: Router::new() }
    }

    pub fn setup_get_route(mut self, name: &str, route: MethodRouter) -> Self {
        self.app = self.app.route(name, route);
        self
    }
}

impl Default for Server {
    fn default() -> Self {
        Server::new()
    }
}
