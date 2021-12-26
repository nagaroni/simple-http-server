use super::server::Handler;
use super::http::{Request, Response, StatusCode, Method};

pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, Some("Home".to_string())),
                _ => Response::new(StatusCode::NotFound, None)
            }
            _ => Response::new(StatusCode::NotFound, None)
        }
    }
}
