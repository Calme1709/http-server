use std::collections::HashMap;

use crate::utils::VecScanner;
use super::headers::{HttpHeaderParser, HttpHeaderValue, HttpHeaders};

#[derive(Debug)]
pub enum UploadedDataParsingError {
	InvalidMimeType,
	InvalidContentDispositionHeader
}

pub struct UploadedData {
	content_type: HttpHeaderValue,
	content_disposition: Option<HttpHeaderValue>,
	underlying: Vec<u8>
}

impl UploadedData {
	pub fn new(
		content_type: HttpHeaderValue,
		content_disposition: Option<HttpHeaderValue>,
		underlying: Vec<u8>
	) -> Self {
		return Self {
			content_type,
			content_disposition,
			underlying
		};
	}

	pub fn as_text(&self) -> Result<String, UploadedDataParsingError> {
		return match self.content_type.as_parameterized_header_value().value.as_str() {
			"text/plain" => Ok(String::from_utf8(self.underlying.clone()).unwrap()),
			_ => Err(UploadedDataParsingError::InvalidMimeType)
		}
	}

	pub fn as_multipart_form_data(&self) -> Result<HashMap<String, UploadedData>, UploadedDataParsingError> {
		if self.content_type.as_parameterized_header_value().value.as_str() != "multipart/form-data" {
			return Err(UploadedDataParsingError::InvalidMimeType);
		}

		let boundary_bytes = match self.content_type.as_parameterized_header_value().parameters.get("boundary") {
			Some(boundary) => format!("--{}", boundary).as_bytes().to_vec(),
			None => return Err(UploadedDataParsingError::InvalidMimeType)
		};

		let mut scanner = VecScanner::new(self.underlying.clone());

		// Discard everything before the first boundary
		scanner.consume_until_pattern([ boundary_bytes.clone(), b"\r\n".to_vec() ].concat().to_vec());

		// If all data has been consumed the body was empty and we can return an empty map
		if scanner.finished() {
			return Ok(HashMap::new());
		}
		
		// Consume the first boundary
		scanner.consume_exact(boundary_bytes.len());
		
		let mut parts: HashMap<String, UploadedData> = HashMap::new();

		loop {
			// If the next two bytes are "--" we have reached the end of the body
			if scanner.peek(2) == b"--" {
				break;
			}

			let header_string = core::str::from_utf8(scanner.consume_until_pattern(b"\r\n\r\n".to_vec()).as_slice()).unwrap().trim().to_string();
			let headers = HttpHeaders::from_string(header_string);

			let content_type = headers.get("Content-Type").unwrap_or(&HttpHeaderValue::new("application/octet-stream".to_string())).clone();

			let content_disposition = match headers.get("Content-Disposition") {
				Option::None => return Err(UploadedDataParsingError::InvalidContentDispositionHeader),
				Option::Some(content_disposition) => content_disposition.clone()
			};

			let name = match content_disposition.as_parameterized_header_value().parameters.get("name") {
				Option::None => return Err(UploadedDataParsingError::InvalidContentDispositionHeader),
				Option::Some(name) => name.clone()
			};

			// Consume newlines
			scanner.consume_exact(4);

			let content = scanner.consume_until_pattern([b"\r\n".to_vec(), boundary_bytes.clone()].concat().to_vec());

			// Consume the boundary
			scanner.consume_exact(boundary_bytes.len() + 2);

			parts.insert(name, UploadedData::new(content_type, Option::Some(content_disposition), content));
		}

		return Ok(parts);
	}

	pub fn as_buffer(&self) -> Vec<u8> {
		return self.underlying.clone();
	}
}

impl std::fmt::Debug for UploadedData {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		return write!(f, "{}", self.as_text().unwrap());
	}
}