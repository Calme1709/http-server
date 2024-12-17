use super::method::HttpMethod;

pub struct HttpRequest {
	pub method: HttpMethod,
	pub path: String,
}
