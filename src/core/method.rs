#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum HttpMethod {
    GET,
    POST,
    DELETE,
    PUT,
    OPTIONS
}



impl HttpMethod {
    pub fn from_str(s: &str) -> HttpMethod {
        match s {
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            "DELETE" => HttpMethod::DELETE,
            "PUT" => HttpMethod::PUT,
            "OPTIONS" => HttpMethod::OPTIONS,
            _ => HttpMethod::GET,
        }
    }

}

impl ToString for HttpMethod {
    fn to_string(&self) -> String {
        let msg = match self {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
            HttpMethod::DELETE => "DELETE",
            HttpMethod::PUT => "PUT",
            HttpMethod::OPTIONS => "OPTIONS",
        };
        msg.to_string()
    }
}