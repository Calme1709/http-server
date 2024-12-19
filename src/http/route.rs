use std::fmt::{
	Display,
	Formatter,
	Result as FormatResult
};

use super::{
	request::HttpRequest,
	response::HttpResponse,
	method::HttpMethod
};

pub type HttpRouteCallback = fn(HttpRequest) -> HttpResponse;

pub struct HttpRoute {
	pub method: HttpMethod,
	pub path_pattern: String,
	pub callback: HttpRouteCallback,
}

impl HttpRoute {
	pub fn matches(&self, request: &HttpRequest) -> bool {
		return self.method == request.method && self.path_matches(&request.uri.path, &self.path_pattern, request.uri.path.len(), self.path_pattern.len());
	}

	fn path_matches(&self, path: &String, pattern: &String, path_index: usize, pattern_index: usize) -> bool {
		// If the pattern is empty it can only match an empty string
		if pattern_index == 0 {
			return path_index == 0;
		}

		// If the path is empty the only matching pattern is a string (potentially zero length) entirely of "*"
		if path_index == 0 {
			for char in pattern.chars() {
				if char != '*' {
					return false;
				}
			}

			return true;
		}

		let path_char = path.chars().collect::<Vec<char>>().get(path_index - 1).unwrap().clone();
		let pattern_char = pattern.chars().collect::<Vec<char>>().get(pattern_index - 1).unwrap().clone();

		// If the current character of the pattern matches the current character of the path - move onto the next character
		if path_char == pattern_char {
			return self.path_matches(path, pattern, path_index - 1, pattern_index - 1)
		}

		// If the current character of the pattern is a '*' there are two cases which could return true:
		//  1. We have completed matching characters for this selector and thus remove it from the pattern and continue
		//  2. We haven't completed matching characters for this selector and thus remove the next character from the path and continue
		if pattern_char == '*' {
			return self.path_matches(path, pattern, path_index, pattern_index - 1) || self.path_matches(path, pattern, path_index - 1, pattern_index);
		}

		return false;
	}
}

impl Display for HttpRoute {
	fn fmt(&self, f: &mut Formatter) -> FormatResult {
		return write!(f, "{} {}", self.method, self.path_pattern);
	}
}