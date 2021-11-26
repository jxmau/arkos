

use std::vec;

use crate::core::{method::HttpMethod, status::StatusCode};

use super::response::Response;


/// A CORSHandler is a struct can will generate a response with specified Headers.
pub struct CORSHandler {
    #[doc(hidden)]
    pub activated: bool,
    origin: Vec<String>,
    methods_allowed: Vec<HttpMethod>,
    headers_allowed: Vec<String>,
    max_age: u32,
}

impl CORSHandler {

    /// Will return a deactivated CORSHandler.
    pub fn inert() -> Self {
        CORSHandler{activated:false, origin: Vec::new(), methods_allowed: Vec::new(), headers_allowed: Vec::new(), max_age: 0u32}
    }


    /// Will return a new, but activated CORSHandler.
    pub fn new() -> Self {
        CORSHandler{activated:true, origin: Vec::new(), methods_allowed: Vec::new(), headers_allowed: Vec::new(), max_age: 0u32}
    }

    /// Will return a CORSHandler preconfigured.
    /// Origin as set to all.
    /// Methods allowed: GET POST DELETE PUT
    /// Headers is empty.
    /// Max Age is set to 86400 seconds. 
    pub fn default() -> Self {
        CORSHandler{activated:true, origin: vec!["*".to_string()], methods_allowed: vec!(HttpMethod::GET, HttpMethod::POST, HttpMethod::PUT, HttpMethod::DELETE), headers_allowed: Vec::new(), max_age: 86400u32}
    }
    
    /// Will set the origins allowed.
    pub fn set_origins(&mut self, origins: Vec<String>) {
        self.origin = origins;
    }

    /// Will set the methods allowed.
    pub fn set_methods_allowed(&mut self, methods: Vec<HttpMethod>) {
        self.methods_allowed = methods;
    }

    /// Will set the headers allowed.
    pub fn set_headers_allowed(&mut self, headers: Vec<String>) {
        self.headers_allowed = headers;
    }

    /// Will set the max age.
    pub fn set_max_age(&mut self, age: u32){
        self.max_age = age;
    }

    /// Will generate a Response with the headers.
    pub fn generate_response(&self) -> Result<Response, StatusCode> {
        let mut response = Response::default();

        let formatter = |v: &Vec<String>| -> String {
            let mut string_returned = "".to_string();
            if !&v.is_empty() {
                for m in v {
                    string_returned.push_str(& format!("{} ", m.to_string()));
                }
                // The last space is erased to not end with a ','
                return string_returned.trim_end().replace(" ", ", ");
            } 
            string_returned
        };

        let methodify = |v: &Vec<HttpMethod>| -> Vec<String> {
            let mut vec_returned = Vec::new();
            for method in v {
                vec_returned.push(method.to_string());
            }
            vec_returned


        };
 
        response.add_header("Access-Control-Allow-Headers".into(), formatter(&self.headers_allowed));
        response.add_header("Access-Control-Allow-Methods".into(), formatter(&methodify(&self.methods_allowed)));
        response.add_header("Access-Control-Max-Age".into(), self.max_age.to_string());
        response.add_header("Access-Control-Allow-Origin".into(), self.origin.first().unwrap().to_string());
        response.convert();
        Ok(response)
    }

}
