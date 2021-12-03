
#![doc = include_str!( "../../docs/cookie.md")]

use log::trace;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cookie {
    pub name: String,
    pub value: String,
    pub expires: String, 
    pub max_age: u32,
    pub path: String,
    pub domain: String,
    pub secure: bool,
    pub http_only: bool,
    pub same_site: String,
}

impl Cookie {

    /// Create a new Cookie with a name and a value
    pub fn new(name: String, value: String) -> Self {
        Cookie { name, value, expires: "".into(), max_age: 0u32, path: "".into(), domain: "".into(), secure: false, http_only: false, same_site: "".into() }
    }

    /// Set the cookie as Secured
    pub fn as_secured(&mut self){
        self.secure = true;
    }
    
    /// Set the cookie as HTTP Only
    pub fn as_http_only(&mut self){
        self.http_only = true;
    }

    /// Set when the cookie is due to expired
    pub fn expires_on(&mut self, exp: String) {
        self.expires = exp;
    }

    pub fn has_max_age(&mut self, age: u32) {
        self.max_age = age; 
    }

    pub fn set_domain(&mut self, domain: String) {
        self.domain = domain;
    }

    pub fn set_path(&mut self, path: String ) {
        self.path = path;
    }

    pub fn set_same_site(&mut self, same_site: String ) {
        self.same_site = same_site;
    }

    #[doc(hidden)]
    pub fn is_valid(&self) -> bool {
        let mut result: bool = true;
        
        // We check if there's any forbiden character - Doesn't allow for the value to be between '"'.
        let char_forbidden = vec!['(', ')', '<', '>', '@', ',', ';', ':', '\\', '\"', '/', '[', ']', '?', '=', '{', '}', ' ' ];
        let mut check = |key: &String, forbiddens: Vec<char>| {
            for c in forbiddens {
                if key.contains(c) {
                    trace!("Cookie of name: {} - character forbidden {} found in name or value. ", self.name, c);
                    result = false;
                }
            }
            if key.contains("   ") {
                trace!("Cookie of name: {} - tabulation found in name or value. ", self.name);
            }
        };
        check(&self.name, char_forbidden);
        check(&self.value, vec!['(', ')', '<', '>', '@', ',', ';', ':', '\\', '\"', '/', '[', ']', '?', '=', '{', '}', ' ' ]);

        // We check the requirement for __Secure- | Unsure Implementation: Might need to ask if the origin is a webpage HTTPS
        if self.name.starts_with("__Secure-") {
            if !self.secure {
                trace!("Cookie of name: {} - Has the prefix __Secure-, but is not secured.", self.name);
                result = false;
            }
        } else if self.name.starts_with("__Host-") {
            if !self.secure {
                trace!("Cookie of name: {} - Has the prefix __Secure-, but is not secured.", self.name);
                result = false;
            }
            if !self.path.eq("/") || self.path.eq("") {
                trace!("Cookie of name: {} - Must not have a path, or a path of \"/\".", self.name);
                result = false;
            }
            if !self.domain.eq("") {
                trace!("Cookie of name: {} - Must not have a specified Domain.", self.name);
                result = false;
            }
        }
        
        result
    }
    


    
    // is_valid()
}


// Test to see if a Cookie is valid or not.
#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn cookie_is_valid(){
        let cookie = Cookie::new("Cookie".into(), "Value".into());
        assert_eq!(cookie.is_valid(), true)
    }
    
    #[test]
    fn cookie_has_forbidden_char(){
        let cookie = Cookie::new("Cookie".into(), "Va/lue".into());
        assert_eq!(cookie.is_valid(), false)
    }

    #[test]
    fn cookie_has_tabulation(){
        let cookie = Cookie::new("Coo   kie".into(), "Value".into());
        assert_eq!(cookie.is_valid(), false)
    }

    #[test]
    fn valid_secure(){
        let mut cookie = Cookie::new("__Secure-Cookie".into(), "Value".into());
        cookie.as_secured();
        assert_eq!(cookie.is_valid(), true)
    }

    #[test]
    fn invalid_secure(){
        let cookie = Cookie::new("__Secure-Cookie".into(), "Value".into());
        assert_eq!(cookie.is_valid(), false)
    }

    #[test]
    fn host_valid(){
        let mut cookie = Cookie::new("__Host-Cookie".into(), "Value".into());
        cookie.set_path("/".into());
        cookie.as_secured();
        assert_eq!(cookie.is_valid(), true)
    }

    #[test]
    fn host_unsecured_with_valid_path(){
        let mut cookie = Cookie::new("__Host-Cookie".into(), "Value".into());
        cookie.set_path("/".into());
        // Domain not specified
        assert_eq!(cookie.is_valid(), false)
    }

    #[test]
    fn host_unsecured_with_invalid_domain(){
        let mut cookie = Cookie::new("__Host-Cookie".into(), "Value".into());
        cookie.set_path("/".into());
        cookie.set_domain("wikipedia.fr".into());
        assert_eq!(cookie.is_valid(), false)
    }

}