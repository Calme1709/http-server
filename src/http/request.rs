use std::collections::HashMap;

use super::method::HttpMethod;
use crate::line_parser::LineParser;

struct RequestLineContent {
	method: HttpMethod,
	path: String,
	version: String
}

#[derive(Debug)]
pub enum HttpRequestParseError {
	UnrecognisedHttpMethod,
	MalformedHeader,
	MalformedRequestLine,
	UnsupportedVersion
}

pub struct HttpRequest {
	pub method: HttpMethod,
	pub path: String,
	pub headers: HashMap<String, String>,
	pub body: Option<String>
}

impl HttpRequest {
	pub fn deserialize_header(request_lines: Vec<String>) -> Result<Self, HttpRequestParseError> {
		let mut parser = LineParser::new(request_lines);

		let request_line_content = Self::deserialize_request_line(parser.consume().unwrap_or(String::from("")))?;

		if request_line_content.version != "HTTP/1.1" {
			return Err(HttpRequestParseError::UnsupportedVersion);
		}

		let mut headers: HashMap<String, String> = HashMap::new();

		loop {
			let line = parser.consume();

			if line.is_none() || line.as_ref().unwrap().is_empty() {
				break;
			}

			let header_parts: Vec<String> = line.unwrap().splitn(2, ":").map(|str| str.parse::<String>().unwrap()).collect::<Vec<String>>();

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
			method: request_line_content.method,
			path: request_line_content.path,
			headers,
			body: Option::None
		});
	}

	fn deserialize_request_line(request_line: String) -> Result<RequestLineContent, HttpRequestParseError> {
		let parts: Vec<String> = request_line.split(" ").map(|part| String::from(part)).collect();

		if parts.len() != 3 {
			return Err(HttpRequestParseError::MalformedRequestLine);
		}

		let method = match HttpMethod::from_string(parts.get(0).unwrap().to_string()) {
			Ok(method) => method,
			Err(_e) => return Err(HttpRequestParseError::UnrecognisedHttpMethod)
		};

		return Ok(RequestLineContent {
			method,
			path: parts.get(1).unwrap().to_string(),
			version: parts.get(2).unwrap().to_string()
		});
	}

	pub fn set_body(&mut self, body: String) -> () {
		self.body = Some(body);
	}
}
