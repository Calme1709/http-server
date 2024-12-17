use std::fmt::{
	Display,
	Formatter,
	Result as FormatResult
};

#[derive(PartialEq, Eq)]
pub enum HttpMethod {
	GET,
}

impl HttpMethod {
	pub fn from_string(string: String) -> Result<Self, String> {
		return match string.as_str() {
			"GET" => Ok(HttpMethod::GET),
			_ => Err(format!("Unrecognised HTTP method {}", string))
		};
	}
}

impl Display for HttpMethod {
	fn fmt(&self, f: &mut Formatter) -> FormatResult {
		let stringified = match self {
			HttpMethod::GET => "GET",
		};

		return write!(f, "{}", stringified);
	}
}