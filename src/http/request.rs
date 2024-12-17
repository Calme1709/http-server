use super::method::HttpMethod;

pub struct HttpRequest {
	pub method: HttpMethod,
	pub path: String,
}

impl HttpRequest {
	pub fn deserialize(request_lines: Vec<String>) -> Result<Self, String> {
		let request_line_parts = request_lines.get(0).unwrap().split(" ").collect::<Vec<&str>>();

		let method = match HttpMethod::from_string(String::from(request_line_parts[0])) {
			Ok(result) => result,
			Err(e) => return Err(e)
		};

		return Ok(Self {
			method,
			path: String::from(request_line_parts[1]),
		});

	}
}
