mod line_parser;
mod http;

use crate::http::server::HttpServer;
use crate::http::response::HttpResponse;

fn main() {
	let mut server = HttpServer::new();

	server.get(
		String::from("/"),
		|request| {
			HttpResponse::new()
				.status(200)
				.header(String::from("Content-Type"), String::from("text/html"))
				.content(format!("Hello, World! From {}", request.headers.get("Host").unwrap()))
		}
	);

	server.post(
		String::from("/"),
		|request| {
			HttpResponse::new()
				.status(200)
				.header(String::from("Content-Type"), String::from("application/json"))
				.content(request.body.unwrap_or(String::from("No body included in request")))
		}
	);

	server.listen(8080);
}
