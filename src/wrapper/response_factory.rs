use std::collections::HashMap;

use crate::{server::{protocol::Protocol, response::Response}, core::{status::StatusCode, status::HttpStatusCode, content::ContentType, method::{HttpMethod}}};

use super::cookie_factory::generate_header;



// Generate a Response Error.
// TODO: Must be compliant by Protocol Version
pub fn generate_response_from_status_code(code: StatusCode) -> Response {
    Response {status: code, content_type: ContentType::Text, headers: HashMap::new(), cookies: Vec::new(), body: "".into()}
}

pub fn convert_response(response: Response, protocol: Protocol, method: HttpMethod) -> String {
    match protocol {
        Protocol::Http1(v) => convert_http1(response, Protocol::Http1(v), method),
        _ => convert_http1(generate_response_from_status_code(StatusCode::MethodNotAllowed), Protocol::Http1(0), method)
    }
}

// Convert a response to a String to be sent back - Needs HTTP Protocol.
fn convert_http1(response: Response, protocol: Protocol, method: HttpMethod) -> String {
    let mut response = response;
    let mut headers = String::new();
    response.headers.insert("Content-Length".to_string(), response.body.len().to_string());
    response.headers.insert("Content-Type".to_string(), response.content_type.get());
    
    headers.push_str(&response.status.generate_headers());
    for (key, val) in response.headers.iter() {
        let entry: String = format!("{}:{}\r\n", key, val);
        headers.push_str(&entry);
    }
    
    for c in &response.cookies { 
        let entry: String = format!("{}: {}\r\n", "Set-Cookie", generate_header(c, &protocol));
        headers.push_str(&entry)
    }

    // 1. Add the First line and the Headers.
    let mut s = format!("HTTP/1.{} {} {}\r\n{}", 
    protocol.get_version(),
    response.status.get_code(), response.status.get_title(),
    headers, 
    );
    // 2. Add the body only if the Method is not HEAD
    if method.eq(&HttpMethod::GET) {
        s.push_str(&format!("\r\n{}", response.body));
    }

    s
}