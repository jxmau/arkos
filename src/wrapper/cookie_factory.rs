use crate::core::cookie::Cookie;
use crate::server::protocol::Protocol;
use log::trace;


pub fn generate_header(cookie: &Cookie, protocol: &Protocol) -> String {
    match protocol {
        Protocol::Http1(v) => generate_header_http1(&cookie, v),
        _  => "".into(),
    }
}




fn generate_header_http1(cookie: &Cookie, _version: &u8) -> String {
    let mut cookie_formated = format!("{}={}", &cookie.name, &cookie.value);
    
    let add = |prefix: &str, to_add: &str| -> String {
        let r = format!("; {}{}", prefix ,to_add);
        r

    };

    if cookie.max_age > 1 { cookie_formated.push_str(&add("Max-Age=", &cookie.max_age.to_string())); }
    if cookie.domain.len() > 1 { cookie_formated.push_str(&add("Domain=", &cookie.domain)); }
    if cookie.expires.len() > 1 { cookie_formated.push_str(&add("Expires=", &cookie.expires)); }
    if cookie.path.len() > 1 { cookie_formated.push_str(&add("Path=", &cookie.path)); }
    if cookie.same_site.len() > 1 { cookie_formated.push_str(&add("SameSite=", &cookie.same_site)); }
    if cookie.http_only { cookie_formated.push_str(&add("", "HttpOnly"));  }
    if cookie.secure { cookie_formated.push_str(&add("","Secure"));  }
    trace!("Set-Cookie header for Cookie of name {} has been generated", cookie.name);
    cookie_formated
}
