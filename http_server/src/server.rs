use std::{
	fs,
	io::Write,
	net::{
		TcpListener,
		TcpStream
	},
	path::Path,
};

use crate::mime_type::MimeType;

use super::{
	HttpMethod,
	HttpRequest,
	HttpResponse,
	HttpRoute,
	HttpRouteCallback,
};

pub struct HttpServer {
	routes: Vec<HttpRoute>,
	static_directories: Vec<String>
}

impl HttpServer {
	pub fn new() -> Self {
		return Self {
			routes: Vec::new(),
			static_directories: Vec::new()
		};
	}

	pub fn get(&mut self, path_pattern: String, callback: HttpRouteCallback) -> () {
		self.routes.push(HttpRoute {
			method: HttpMethod::GET,
			path_pattern,
			callback
		});
	}

	pub fn post(&mut self, path_pattern: String, callback: HttpRouteCallback) -> () {
		self.routes.push(HttpRoute {
			method: HttpMethod::POST,
			path_pattern,
			callback
		});
	}

	pub fn serve_static(&mut self, directory_path: String) -> () {
		self.static_directories.push(directory_path);
	}

	pub fn listen(&self, port: u16) -> () {
		let tcp_listener = TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap();

		for stream in tcp_listener.incoming().map(|stream| stream.unwrap()) {
			self.handle_connection(stream);
		}
	}

	fn handle_connection(&self, mut stream: TcpStream) -> () {
		let response = match HttpRequest::from_stream(&mut stream) {
			Ok(request) => self.handle_request(request),
			Err(_e) => HttpResponse::new().status(400)
		};

		stream.write_all(response.serialize().as_bytes()).unwrap();
	}

	fn handle_request(&self, request: HttpRequest) -> HttpResponse {
		// Find the matching route and return the result of the callback
		for route in &self.routes {
			if route.matches(&request) {
				return (route.callback)(request);
			}
		}

		if request.method == HttpMethod::GET {
			for static_directory in &self.static_directories {
				let path = Path::new(&static_directory).join(&request.uri.path.strip_prefix("/").unwrap_or(&request.uri.path));

				// TODO: Support index files
				if path.exists() && path.is_file() {
					let content_or_error = fs::read_to_string(path);

					return match content_or_error {
						Ok(content) => HttpResponse::new()
							.status(200)
							.header(String::from("Content-Type"), MimeType::from_file_path(request.uri.path))
							.content(content),
						Err(_e) => HttpResponse::new().status(500)
					}
				}
			}
		}

		// If there are no matching routes - return a 404 error
		return HttpResponse::new().status(404);
	}
}
