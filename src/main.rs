mod utils;
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

	server.get(
		String::from("/params"),
		|request| {
			HttpResponse::new()
				.status(200)
				.header(String::from("Content-Type"), String::from("application/json"))
				.content(format!("{:#?}", request.uri.query))
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

	server.get(
		String::from("/documents/*.txt"),
		|request| {
			HttpResponse::new()
				.status(200)
				.header(String::from("Content-Type"), String::from("text/plain"))
				.content(format!("{}", request.uri.path))
		}
	);

	server.listen(8080);
}
