use std::collections::HashMap;

use crate::{server::{protocol::Protocol, response::Response}, core::{status::StatusCode, status::HttpStatusCode, content::ContentType}};

use super::cookie_factory::generate_header;



// Generate a Response Error.
// TODO: Must be compliant by Protocol Version
pub fn generate_response_from_status_code(code: StatusCode) -> Response {
    Response {status: code, content_type: ContentType::Text, headers: HashMap::new(), cookies: Vec::new(), body: "".into()}
}

pub fn convert_response(response: Response, protocol: Protocol) -> String {
    match protocol {
        Protocol::Http1(v) => convert_http1(response, Protocol::Http1(v)),
        _ => convert_http1(generate_response_from_status_code(StatusCode::MethodNotAllowed), Protocol::Http1(0))
    }
}

// Convert a response to a String to be sent back - Needs HTTP Protocol.
fn convert_http1(response: Response, protocol: Protocol) -> String {
    let mut response = response;
    let mut headers = String::new();
    response.headers.insert("Content-Length".to_string(), response.body.len().to_string());
    response.headers.insert("Content-Type".to_string(), response.content_type.get());
    
    for (key, val) in response.headers.iter() {
        let entry: String = format!("{}:{}\r\n", key, val);
        headers.push_str(&entry);
    }
    
    for c in &response.cookies { 
        let entry: String = format!("{}: {}\r\n", "Set-Cookie", generate_header(c, &protocol));
        headers.push_str(&entry)
    }

    let s : String = format!("HTTP/1.1 {} {}\r\n{}\r\n{}", 
        response.status.get_code(), response.status.get_title(),
        headers, 
        response.body
    ) ;
    s
}