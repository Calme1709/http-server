use std::io::{BufRead, BufReader, Read};

use super::{
	headers::{HttpHeaderParser, HttpHeaderValue, HttpHeaders}, uploaded_data::UploadedData, uri::URI
};

use crate::HttpMethod;

#[derive(Debug)]
pub enum HttpRequestParseError {
	UnrecognisedHttpMethod,
	MalformedHeader,
	MalformedRequestLine,
	UnsupportedVersion
}

pub struct HttpRequest {
	pub method: HttpMethod,
	pub uri: URI,
	pub headers: HttpHeaders,
	pub body: Option<UploadedData>
}

impl HttpRequest {
    pub fn from_stream(stream: &mut std::net::TcpStream) -> Result<Self, HttpRequestParseError> {
        let mut buffer_reader = BufReader::new(stream);

        let mut request_line = String::new();
        buffer_reader.read_line(&mut request_line).unwrap();

        let (method, uri, version) = match Self::deserialize_request_line(request_line) {
            Ok((method, url, version)) => (method, url, version),
			Err(e) => return Err(e)
        };

        if version.as_str() != "HTTP/1.1" {
            return Err(HttpRequestParseError::UnsupportedVersion);
        }

		let mut header_string = String::new();

		loop {
			let mut line = String::new();
			buffer_reader.read_line(&mut line).unwrap();

			if line == "\r\n" {
				break;
			}

			header_string.push_str(&line);
		}
		
		let headers = HttpHeaders::from_string(header_string);

		let mut body = Option::None;

        if headers.contains_key("Content-Length") {
            let content_length = headers.get("Content-Length").unwrap().as_parameterized_header_value().value.parse::<usize>().unwrap();
            let mut body_content = vec![0; content_length];
            buffer_reader.read_exact(&mut body_content).unwrap();

			let content_type = headers.get("Content-Type").unwrap_or(&HttpHeaderValue::new("application/octet-stream".to_string())).clone();
			let content_disposition = headers.get("Content-Disposition").and_then(|header| Some(header.clone()));

			body = Some(
				UploadedData::new(
					content_type,
					content_disposition,
					body_content
				)
			);
        }

		return Ok(Self {
			method,
			uri,
			headers,
			body
		});
    }

	fn deserialize_request_line(request_line: String) -> Result<(HttpMethod, URI, String), HttpRequestParseError> {
		let parts: Vec<String> = request_line.trim().split(" ").map(|part| String::from(part)).collect();

		if parts.len() != 3 {
			return Err(HttpRequestParseError::MalformedRequestLine);
		}

		let uri = URI::from_string(parts.get(1).unwrap().to_string());

		let method = match HttpMethod::from_string(parts.get(0).unwrap().to_string()) {
			Ok(method) => method,
			Err(_e) => return Err(HttpRequestParseError::UnrecognisedHttpMethod)
		};

		return Ok(( method, uri, parts.get(2).unwrap().to_string() ));
	}
}
