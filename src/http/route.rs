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
	pub path: String,
	pub callback: HttpRouteCallback,
}

impl Display for HttpRoute {
	fn fmt(&self, f: &mut Formatter) -> FormatResult {
		return write!(f, "{} {}", self.method, self.path);
	}
}