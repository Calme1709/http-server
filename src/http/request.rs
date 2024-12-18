use std::collections::HashMap;

use super::method::HttpMethod;

pub enum HttpRequestParseError {
	UnrecognisedHttpMethod,
	MalformedHeader,
}

pub struct HttpRequest {
	pub method: HttpMethod,
	pub path: String,
	pub headers: HashMap<String, String>,
}

impl HttpRequest {
	pub fn deserialize(request_lines: Vec<String>) -> Result<Self, HttpRequestParseError> {
		let request_line_parts = request_lines.get(0).unwrap().split(" ").collect::<Vec<&str>>();

		let method = match HttpMethod::from_string(String::from(request_line_parts[0])) {
			Ok(result) => result,
			Err(_e) => return Err(HttpRequestParseError::UnrecognisedHttpMethod)
		};

		let mut headers: HashMap<String, String> = HashMap::new();

		for line in request_lines.iter().take_while(|line| !line.is_empty()).skip(1) {
			let header_parts: Vec<String> = line.splitn(2, ":").map(|str| str.parse::<String>().unwrap()).collect::<Vec<String>>();

			let key = match header_parts.get(0) {
				Some(key) => String::from(key),
				None => return Err(HttpRequestParseError::MalformedHeader)
			};

			let value = match header_parts.get(1) {
				Some(value) => String::from(value.trim_start()),
				None => return Err(HttpRequestParseError::MalformedHeader)
			};

			headers.insert(key, value);
		}

		return Ok(Self {
			method,
			path: String::from(request_line_parts[1]),
			headers
		});

	}
}
