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
		let response: HttpResponse = match self.parse_request(request_text_lines) {
			// Handle the request based on the based struct
			Ok(request) => self.handle_request(request),
			
			// If we are unable to parse the request return a 400 Bad Request
			Err(_err) => HttpResponse {
				status: 400,
				status_text: String::from("Bad Request"),
				content: Option::None
			}
		};

		let status_line = format!("HTTP/1.1 {} {}", response.status, response.status_text);

		let mut headers = Vec::new();

		let content = response.content.unwrap_or(String::from(""));

		headers.push(format!("Content-Length: {}", content.len()));

		let headers_string = headers.join("\r\n");

		stream.write_all(format!("{status_line}\r\n{headers_string}\r\n\r\n{content}").as_bytes()).unwrap();
	}

	fn parse_request(&self, request_text_lines: Vec<String>) -> Result<HttpRequest, String> {
		let request_line_parts = request_text_lines.get(0).unwrap().split(" ").collect::<Vec<&str>>();

		let method = match HttpMethod::from_string(String::from(request_line_parts[0])) {
			Ok(result) => result,
			Err(e) => return Err(e)
		};

		return Ok(HttpRequest {
			method,
			path: String::from(request_line_parts[1]),
		});
	}

	fn handle_request(&self, request: HttpRequest) -> HttpResponse {
		// Find the matching route and return the result of the callback
		for route in &self.routes {
			if route.method == request.method && route.path == request.path {
				return (route.callback)(request);
			}
		}

		// If there are no matching routes - return a 404 error
		return HttpResponse {
			status: 404,
			status_text: String::from("Not Found"),
			content: Option::None,
		}
	}
}
