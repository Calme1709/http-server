mod http;

use crate::http::server::HttpServer;
use crate::http::response::HttpResponse;

fn main() {
	let mut server = HttpServer::new();

	server.get(
		String::from("/"),
		|_request|
			HttpResponse::new()
				.status(200)
				.content(String::from("Hello, World!"))
	);

	server.listen(8080);
}
