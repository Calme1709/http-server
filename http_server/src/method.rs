use std::fmt::{
	Display,
	Formatter,
	Result as FormatResult
};

#[derive(PartialEq, Eq)]
pub enum HttpMethod {
	GET,
	POST,
}

impl HttpMethod {
	pub fn from_string(string: String) -> Result<Self, String> {
		return match string.as_str() {
			"GET" => Ok(HttpMethod::GET),
			"POST" => Ok(HttpMethod::POST),
			_ => Err(format!("Unrecognised HTTP method {}", string))
		};
	}
}

impl Display for HttpMethod {
	fn fmt(&self, f: &mut Formatter) -> FormatResult {
		let stringified = match self {
			HttpMethod::GET => "GET",
			HttpMethod::POST => "POST"
		};

		return write!(f, "{}", stringified);
	}
}