
#![doc = include_str!( "../../docs/route.md")]


use std::{collections::HashMap, sync::Arc};

use log::debug;

use crate::core::{method::HttpMethod, status::StatusCode};

use super::{request::Request, response::Response};


#[derive(Clone)]
pub struct Route{
    #[doc(hidden)]
    pub url : String,
    #[doc(hidden)]
    pub method: HttpMethod,
    #[doc(hidden)]
    pub request: Option<Request>,
    #[doc(hidden)]
    pub required_param: Vec<String>,
    #[doc(hidden)]
    pub required_header: Vec<String>,
    #[doc(hidden)]
    pub required_cookie: Vec<String>,
    #[doc(hidden)]
    pub response: Arc<dyn Fn(Request) -> Result<Response, StatusCode> + Send + Sync>,
    #[doc(hidden)]
    pub checks: Vec<Arc<dyn Fn(Request) -> Result<(), StatusCode> + Send + Sync>>,
}

impl Route {
    
    /// Will create a new Route with a path and a HTTP Method, but with an empty Response that will return a 200 Ok if called.
    pub fn new(url: &str,method: HttpMethod) -> Self{
        Route {url : url.to_string(), method, request: None,  required_param: Vec::new(), required_header: Vec::new(),required_cookie: Vec::new(),  response : Arc::new(|_req: Request| {Ok(Response::default())}), checks: Vec::new() }
    }

    /// Will add a required url parameters. If missing, the server will return a 400 Bad Request Response.
    pub fn add_required_url_param(&mut self,  name: &str) -> &mut Self {
        self.required_param.push(name.into());
        self
    }

    /// Will add a required header. If missing, the server will return a 400 Bad Request Response.
    pub fn add_required_header(&mut self,  name: &str) -> &mut Self {
        self.required_header.push(name.into());
        self
    }

    /// Will add a required cookie. If missing, the server will return a 400 Bad Request Response.
    pub fn add_required_cookie(&mut self,  name: &str) -> &mut Self {
        self.required_cookie.push(name.into());
        self
    }

    /// Will set a Response to the Route. If you want to return only a Status Code like 401 or 403, use Err(StatusCode::Unauthorized) instead. The server will generate a Response from it when calling your closure.
    pub fn set_response<'a>(&mut self, fun: Arc<dyn Fn(Request) -> Result<Response, StatusCode> + Send + Sync>)  {
        self.response = fun;
    }
    
    pub fn add_check(&mut self, check: Arc<dyn Fn(Request) -> Result<(), StatusCode> + Send + Sync>)  {
        self.checks.push(check);
    }
        
    /// Will iterated throught every required fields to know if the Request is valid.
    /// Will tell you the missing field in the console if the debug level is allowed.
    pub fn is_request_valid(&self, request: &Request) -> bool {

        let checker = |item: String, required: &Vec<String>, list: &HashMap<String, String>| -> bool {
            if !required.is_empty() {
                if list.is_empty(){
                    return false;
                } 
                for header in required {
                    if !list.contains_key(header) {
                        debug!("{} {} is missing.", item, &header);
                        return false;
                    } 
                }
            }
            true
        };

        if !checker("Header".into(), &self.required_header, &request.headers) || 
            !checker("Param".into(), &self.required_param, &request.param) ||
            !checker("Cookie".into(), &self.required_cookie, &request.cookies) {
            return false;
        }

        true
    }
}

