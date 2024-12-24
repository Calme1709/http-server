use std::{
	collections::HashMap,
	fmt::{Debug, Display, Formatter, Result as FormatResult}
};

use crate::utils::VecScanner;

pub struct ParameterizedHeaderValue {
	pub value: String,
	pub parameters: HashMap<String, String>
}

#[derive(Clone)]
pub struct HttpHeaderValue(String);

impl Display for HttpHeaderValue {
	fn fmt(&self, f: &mut Formatter) -> FormatResult {
		return write!(f, "{}", self.0);
	}
}

impl Debug for HttpHeaderValue {
	fn fmt(&self, f: &mut Formatter) -> FormatResult {
		return write!(f, "{:?}", self.0);
	}
}

impl HttpHeaderValue {
	pub fn new(value: String) -> Self {
		return Self(value);
	}

	pub fn as_parameterized_header_value(&self) -> ParameterizedHeaderValue {
		let mut scanner = VecScanner::new(self.0.chars().collect::<Vec<char>>());

		let value = scanner.consume_until_value(';').into_iter().collect::<String>();

		let mut parameters: HashMap<String, String> = HashMap::new();

		while !scanner.finished() {
			// Consume the semicolon
			scanner.consume_exact(1);

			// Consume whitespace
			scanner.consume_until(|char, _index| char != ' ' && char != '\t');

			// Consume parameter definition
			let entry = scanner.consume_until_value(';');

			let mut entry_scanner = VecScanner::new(entry);

			let key = entry_scanner.consume_until_value('=').into_iter().collect::<String>().trim().to_string();
			entry_scanner.consume_exact(1);
			let mut value = entry_scanner.consume_rest().into_iter().collect::<String>().trim().to_string();

			// Remove enclosing quotes from value if present
			if value.starts_with("\"") && value.ends_with("\"") {
				value = value[1..value.len() - 1].to_string();
			}

			parameters.insert(key, value);
		}

		return ParameterizedHeaderValue {
			value,
			parameters
		};
	}
}

pub type HttpHeaders = HashMap<String, HttpHeaderValue>;

pub trait HttpHeaderParser {
	fn from_string(header: String) -> Self;
}

impl HttpHeaderParser for HashMap<String, HttpHeaderValue> {
	fn from_string(header: String) -> Self {
		let mut headers: HttpHeaders = HashMap::new();

		let mut scanner = VecScanner::new(header.chars().collect::<Vec<char>>());

		while !scanner.finished() {
			let line = scanner.consume_until_pattern("\r\n".chars().collect::<Vec<char>>());

			// Consume whitespace
			scanner.consume_exact(2);

			let mut line_scanner = VecScanner::new(line);

			let key = line_scanner.consume_until_value(':').into_iter().collect::<String>().trim().to_string();
			line_scanner.consume_exact(1);
			let value = line_scanner.consume_rest().into_iter().collect::<String>().trim().to_string();

			headers.insert(key, HttpHeaderValue::new(value));
		}

		return headers;
	}
}
