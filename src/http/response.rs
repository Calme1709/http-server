pub struct HttpResponse {
	pub status: u16,
	pub status_text: String,
	pub content: Option<String>
}

impl HttpResponse {
	pub fn new() -> Self {
		return HttpResponse {
			status: 200,
			status_text: String::from("OK"),
			content: Option::None
		};
	}

	pub fn status(mut self, status: u16) -> Self {
		self.status = status;
		self.status_text = match status {
			100 => String::from("Continue"),
			101 => String::from("Switching Protocols"),

			200 => String::from("OK"),
			201 => String::from("Created"),
			202 => String::from("Accepted"),
			203 => String::from("Non-Authoritative Information"),
			204 => String::from("No Content"),
			205 => String::from("Reset Content"),
			206 => String::from("Partial Content"),

			300 => String::from("Multiple Choices"),
			301 => String::from("Moved Permanently"),
			302 => String::from("Found"),
			303 => String::from("See Other"),
			304 => String::from("Not Modified"),
			305 => String::from("Use Proxy"),
			307 => String::from("Temporary Redirect"),
			308 => String::from("Permanent Redirect"),

			400 => String::from("Bad Request"),
			401 => String::from("Unauthorized"),
			402 => String::from("Payment Required"),
			403 => String::from("Forbidden"),
			404 => String::from("Not Found"),
			405 => String::from("Method Not Allowed"),
			406 => String::from("Not Acceptable"),
			407 => String::from("Proxy Authentication Required"),
			408 => String::from("Request Timeout"),
			409 => String::from("Conflict"),
			410 => String::from("Gone"),
			411 => String::from("Length Required"),
			412 => String::from("Precondition Failed"),
			413 => String::from("Content Too Large"),
			414 => String::from("URI Too Long"),
			415 => String::from("Unsupported Media Type"),
			416 => String::from("Range Not Satisfiable"),
			417 => String::from("Expectation Failed"),
			421 => String::from("Misdirected Request"),
			422 => String::from("Unprocessable Content"),
			426 => String::from("Upgrade Required"),

			500 => String::from("Internal Server Error"),
			501 => String::from("Not Implemented"),
			502 => String::from("Bad Gateway"),
			503 => String::from("Service Unavailable"),
			504 => String::from("Gateway Timeout"),
			505 => String::from("HTTP Version Not Supported"),

			_ => todo!()
		};

		return self;
	}

	pub fn content(mut self, content: String) -> Self {
		self.content = Option::Some(content);

		return self;
	}
}
