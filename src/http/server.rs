use std::{
	io::{
		BufRead,
		BufReader,
		Write
	},
	net::{
		TcpListener,
		TcpStream
	}
};

use super::{
	method::HttpMethod,
	request::HttpRequest,
	response::HttpResponse,
	route::HttpRoute,
	route::HttpRouteCallback,
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

	pub fn get(&mut self, path: String, callback: HttpRouteCallback) -> () {
		self.routes.push(HttpRoute {
			method: HttpMethod::GET,
			path,
			callback
		});
	}

	pub fn listen(&self, port: u16) -> () {
		let tcp_listener = TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap();

		for stream in tcp_listener.incoming() {
			self.handle_connection(stream.unwrap());
		}
	}

	fn handle_connection(&self, mut stream: TcpStream) -> () {
		let request_text_lines: Vec<String> = BufReader::new(&stream)
			.lines()
			.map(|line| line.unwrap())
			.take_while(|line| !line.is_empty())
			.collect();
		
		// Parse the request into a struct which we can use
		let response: HttpResponse = match HttpRequest::deserialize(request_text_lines) {
			// Handle the request based on the based struct
			Ok(request) => self.handle_request(request),
			
			// If we are unable to parse the request return a 400 Bad Request
			Err(_err) => HttpResponse::new().status(400)
		};

		stream.write_all(response.serialize().as_bytes()).unwrap();
	}

	fn handle_request(&self, request: HttpRequest) -> HttpResponse {
		// Find the matching route and return the result of the callback
		for route in &self.routes {
			if route.method == request.method && route.path == request.path {
				return (route.callback)(request);
			}
		}

		// If there are no matching routes - return a 404 error
		return HttpResponse::new().status(400);
	}
}
