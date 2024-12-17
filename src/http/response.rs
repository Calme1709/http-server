pub struct HttpResponse {
	pub status: u16,
	pub status_text: String,
	pub content: Option<String>
}
