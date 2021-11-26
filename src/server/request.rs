use std::collections::HashMap;


use std::result::Result;

use crate::core::status::StatusCode;
use crate::core::method::HttpMethod;

#[derive(Debug, Clone)]
pub struct Request {

    #[doc(hidden)]
    pub method: HttpMethod,
    #[doc(hidden)]
    pub url: String,
    #[doc(hidden)]
    pub headers : HashMap<String, String>,
    #[doc(hidden)]
    pub cookies : HashMap<String, String>,
    #[doc(hidden)]
    pub param : HashMap<String, String>,
    #[doc(hidden)]
    pub body : String,
}

impl Request {

    /// Will parse a HTTP/1.1 request into a Request struct. If it fails, it will return a 500 Internal Server Error Response.

    pub fn parse(raw_request: &str) -> Result<Request, StatusCode> {
        // Firs thing we do is check to see if we have the `body` separator: \r\n\r\n
        // and if we do, we take all the bytes up until the body separator.
        //
        // We also keep `pos` around so we can get the `body` at the end, by starting
        // from the `pos` and taking the remaining bytes
        let pos = raw_request.find("\r\n\r\n").unwrap_or(raw_request.len());

        // Create a line iterator, meaning for every iteration we are looking at a new line
        let mut parsed = raw_request[..pos].lines();

        // Get the method and path iterator
        let mut method_and_path = parsed.next().map(|line| line.split(' ')).ok_or(StatusCode::InternalServerError)?;
        // ... and then get the method
        let method = HttpMethod::from_str(method_and_path.next().ok_or(StatusCode::InternalServerError)?);
        // ... and finally the path
        let mut url_and_param = method_and_path.next().map(|line| line.split('?')).ok_or(StatusCode::InternalServerError)?;

        let url = url_and_param.next().ok_or(StatusCode::InternalServerError)?;

        let params : HashMap<String, String> = match url_and_param.next() {
            Some(param_line) => {
                let mut params = HashMap::new();
                for mut param in param_line.split('&').map(|c| c.splitn(2, '=')) {
                    if let (Some(key), Some(value)) = (param.next(), param.next()) {
                        params.insert(key.into(), value.into());
                    }
                };
                params
            },
            None => HashMap::new(),
        };
        
        


        let mut headers: HashMap<String, String> = HashMap::new();
        let mut cookies: HashMap<String, String> = HashMap::new();

        // Cookie: yummy_cookie=choco; tasty_cookie=strawberry;
        for row in parsed {
            match row.strip_prefix("Cookie: ") {
                Some(cookie_data) => {
                    for mut data in cookie_data.split(';').map(|c| c.splitn(2, '=')) {
                        if let (Some(key), Some(val)) = (data.next(), data.next()) {
                            cookies.insert(key.into(), val.into());
                        }
                    }
                }
                None => {
                    let mut header_data = row.split(":");
                    if let (Some(name), Some(value)) = (header_data.next(), header_data.next()) {
                        headers.insert(name.into(), value.into());
                    }
                }
            }
        }
        
        let body_start = (pos + 4).min(raw_request.len());
        let body = raw_request[body_start..].to_string();
        Ok(Request {method, url: url.to_string(), headers, cookies, param: params, body})
    }

}