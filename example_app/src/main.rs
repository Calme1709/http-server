use http_server::{HttpServer, HttpResponse};

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


	server.get(
		String::from("/documents/*.txt"),
		|request| {
			HttpResponse::new()
				.status(200)
				.header(String::from("Content-Type"), String::from("text/plain"))
				.content(format!("{}", request.uri.path))
		}
	);

	server.serve_static(String::from("/workspaces/http-server/example_app/public/"));

	server.post(String::from("/upload"), |request| {
		println!("{:#?}", request.headers);
		println!("{:#?}", request.body.unwrap().as_multipart_form_data().unwrap());

		HttpResponse::new()
			.status(200)
			.header(String::from("Content-Type"), String::from("text/plain"))
			.content(String::from("File uploaded successfully"))
	});

	server.listen(8080);
}
