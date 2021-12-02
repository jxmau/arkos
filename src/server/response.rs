use std::collections::HashMap;


use crate::{core::{content::ContentType, status::{StatusCode}, cookie::Cookie}};





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



}

