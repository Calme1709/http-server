use std::{
	io::{
		BufRead,
		BufReader,
		Read,
		Write
	},
	net::{
		TcpListener,
		TcpStream
	}
};

use super::{
	HttpMethod,
	HttpRequest,
	HttpResponse,
	HttpRoute,
	HttpRouteCallback,
};

pub struct HttpServer {
	routes: Vec<HttpRoute>
}

impl HttpServer {
	pub fn new() -> Self {
		return Self {
			routes: Vec::new()
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

	pub fn listen(&self, port: u16) -> () {
		let tcp_listener = TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap();

		for mut stream in tcp_listener.incoming().map(|stream| stream.unwrap()) {
			let response = self.handle_connection(&stream);

			stream.write_all(response.serialize().as_bytes()).unwrap();
		}
	}

	fn handle_connection(&self, stream: &TcpStream) -> HttpResponse {
		let mut buffer_reader = BufReader::new(stream);

		let mut request_header: Vec<String> = Vec::new();
		
		// Read the header
		loop {
			let mut line = String::new();
		
			buffer_reader.read_line(&mut line).unwrap();

			if line == "\r\n" {
				break;
			}

			request_header.push(line.trim().to_string());
		}

		let mut request = match HttpRequest::deserialize_header(request_header) {
			Ok(request) => request,
			Err(_err) => return HttpResponse::new().status(400)
		};
		
		if request.headers.contains_key("Content-Length") {
			let content_length = request.headers.get("Content-Length").unwrap().parse::<usize>().unwrap();

			let mut body_bytes = vec![0; content_length];

			buffer_reader.read_exact(&mut body_bytes).unwrap();

			request.set_body(core::str::from_utf8(&body_bytes).unwrap().to_string());
		}
		
		return self.handle_request(request);
	}

	fn handle_request(&self, request: HttpRequest) -> HttpResponse {
		// Find the matching route and return the result of the callback
		for route in &self.routes {
			if route.matches(&request) {
				return (route.callback)(request);
			}
		}

		// If there are no matching routes - return a 404 error
		return HttpResponse::new().status(404);
	}
}
