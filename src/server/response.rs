use std::collections::HashMap;


use crate::core::{content::ContentType, status::{StatusCode, HttpStatusCode}};

use super::cookie::Cookie;



#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Response {
    #[doc(hidden)]
    pub status: StatusCode,
    #[doc(hidden)]
    pub content_type: ContentType,
    #[doc(hidden)]
    pub headers: HashMap<String, String>,
    #[doc(hidden)]
    pub cookies: Vec<Cookie>,
    #[doc(hidden)]
    pub body: String,
}

impl Response {

    /// Will return a Response with an empty body, a Ok Response and a Content-Type of application/json.
    pub fn default() -> Self {
        Response {status: StatusCode::Ok, content_type: ContentType::Json, headers: HashMap::new(), cookies: Vec::new(), body: "".to_string()}
    }
    
    /// To use to set the body of the Response.
    pub fn set_body(&mut self, body: String) -> &mut Response {
        self.body = body;
        self
    }

    /// Will add a header to the Response. 
    /// Don't specify Content-Length. If you want to specify a Content-Type, use `set_content_type' instead.
    pub fn add_header(&mut self, key: String, val: String) {
        self.headers.insert(key, val);
    }

    /// Add a Cookie that will be return with the Response.
    pub fn add_cookie(&mut self, cookie: Cookie) {
        self.cookies.push(cookie);
    }

    /// Will specify the Content-Type Header for the Response.
    /// In case of a enum variant missing, use the variant `Custom(String)`
    pub fn set_content_type(&mut self, content_type: ContentType){
        self.content_type = content_type;
    }


    #[doc(hidden)]
    // Generate a Response Error.
    pub fn generate_from_status_code(code: StatusCode) -> Self {
        Response {status: code, content_type: ContentType::Text, headers: HashMap::new(), cookies: Vec::new(), body: "".into()}
    }

    #[doc(hidden)]
    pub fn convert(&mut self) -> String {

        let mut headers = String::new();
        self.headers.insert("Content-Length".to_string(), self.body.len().to_string());
        self.headers.insert("Content-Type".to_string(), self.content_type.get());
        
        for (key, val) in self.headers.iter() {
            let entry: String = format!("{}:{}\r\n", key, val);
            headers.push_str(&entry);
        }
        
        for c in &self.cookies { 
            let entry: String = format!("{}: {}\r\n", "Set-Cookie", c.generate_header());
            headers.push_str(&entry)
        }

        let s : String = format!("HTTP/1.1 {} {}\r\n{}\r\n{}", 
            self.status.get_code(), self.status.get_title(),
            headers, 
            self.body
        ) ;
        s
    }
}

