use std::collections::HashMap;

use crate::{server::{protocol::Protocol, response::Response}, core::{status::StatusCode, status::HttpStatusCode, content::ContentType, method::{HttpMethod}}};

use super::cookie_factory::generate_header;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResponseFactory {
    protocol: Protocol,
    method: HttpMethod,
    pub response: Response, // Only so the Response can be tested.
    following_response: Vec<ResponseFactory>, // In the case of upgrades 
}


impl ResponseFactory {
    
    // StatusCode
    pub fn for_status_code(protocol: Protocol, code: StatusCode) -> Self {
        let response = Response {status: code, content_type: ContentType::Text, headers: HashMap::new(), cookies: Vec::new(), body: "".into()};
        ResponseFactory {protocol, method: HttpMethod::GET, response, following_response: Vec::new()}
    }

    // New | TODO: Should the follow up response put here? 
    pub fn new(protocol: Protocol, method: HttpMethod, response: Response) -> Self {
        ResponseFactory {protocol, method, response, following_response: Vec::new() }
    }
    
    pub fn add_followup_response_factory(&mut self, factory: ResponseFactory) {
        self.following_response.push(factory);
    }

    // Consume
    pub fn consume(&mut self) -> String {
        match &self.protocol {
            Protocol::Http1(v) => convert_http1(&mut self.response, Protocol::Http1(*v), self.method),
            _ => convert_http1(&mut self.response, Protocol::Http1(0), self.method)
        }
    }
    
    // Convert HTTP1.x



}   


// // Generate a Response Error.
// // TODO: Must be compliant by Protocol Version
// pub fn generate_response_from_status_code(code: StatusCode) -> Response {
//     Response {status: code, content_type: ContentType::Text, headers: HashMap::new(), cookies: Vec::new(), body: "".into()}
// }

// pub fn convert_response(response: Response, protocol: Protocol, method: HttpMethod) -> String {
//     match protocol {
//         Protocol::Http1(v) => convert_http1(response, Protocol::Http1(v), method),
//         _ => convert_http1(generate_response_from_status_code(StatusCode::MethodNotAllowed), Protocol::Http1(0), method)
//     }
// }

// Convert a response to a String to be sent back - Needs HTTP Protocol.
fn convert_http1(response: &mut Response, protocol: Protocol, method: HttpMethod) -> String {
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