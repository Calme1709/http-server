mod http;

use crate::http::server::HttpServer;
use crate::http::response::HttpResponse;

fn main() {
	let mut server = HttpServer::new();

	server.get(
		String::from("/"),
		|_request| HttpResponse {
			status: 200,
			status_text: String::from("OK"),
			content: Option::Some(String::from("Hello, World!"))
		}
	);

	server.listen(8080);
}
