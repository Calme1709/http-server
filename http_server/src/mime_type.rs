pub struct MimeType {}

impl MimeType {
    pub fn from_file_path(file_path: String) -> String {
        let file_name = match file_path.contains("/") {
            true => file_path.split("/").last().unwrap(),
            false => &file_path
        };

        let extension = match file_name.contains(".") {
            true => file_name.split(".").last().unwrap(),
            false => ""
        };

        return match extension {
            "txt" => String::from("text/plain",),
            "html" => String::from("text/html"),

            _ => String::from("application/octet-stream")
        };
    }
}