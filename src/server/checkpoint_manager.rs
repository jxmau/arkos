use log::warn;

use crate::core::status::StatusCode;

use super::{checkpoint::{Checkpoint}, request::Request};



pub struct CheckpointManager {
    pub checkpoint: Checkpoint,
}

impl CheckpointManager{
    
    // new
    pub fn new(checkpoint: Checkpoint) -> Self {
        CheckpointManager { checkpoint }
    }


    // CheckMethod
    pub fn verify(&self, req: Request) -> Option<StatusCode> {
        if self.checkpoint.paths.len() == 0 {
            warn!("Checkpoint has been declared, but no path has been found.");
            return None;
        }
        for path in &self.checkpoint.paths {
            if path.eq(&format!("!{}", req.url)){
                return None;
            } else if req.url.eq(path) // Same path - Explicit same path
                || ( path.ends_with("**") && req.url.starts_with(path.strip_suffix("**").unwrap()) ) // Base allowed - Check if the path listed ends with ** and the request starts with path
                || ( path.ends_with("*") && req.url.starts_with(path.strip_suffix("*").unwrap()) && compare(&path, &req.url) )
            {
                match (&self.checkpoint.check)(req.clone()) {
                    Ok(_) => continue,
                    Err(e) => return Some(e),
                }
            }
        }
        None
    }
}


pub fn compare(list: &str, req: &str) -> bool {
    // list : /hello/*  req: /hello/hello
    if req.ends_with('/') {
        let req = req.strip_suffix('/').unwrap();
        return list.matches('/').count() == req.matches('/').count();
    }
    list.matches('/').count() == req.matches('/').count()
}


// Redirection Test
#[cfg(test)]
mod test {
    
    use std::{sync::Arc, collections::HashMap};

    use crate::{core::method::HttpMethod};

    use super::*;
    
    // The check should always return a StatusCode.
    // Send - Path in Checkpoint - Expected

    fn base(list: &str) -> CheckpointManager {
        let checkpoint = Checkpoint::new(vec!(list.into(), "/".into()), Arc::new(|_req: Request| {Err(StatusCode::BadRequest)} ));
        CheckpointManager::new(checkpoint)
    }

    fn base_req(path: &str ) -> Request {
        Request { method: HttpMethod::GET, url: path.into(), headers: HashMap::new(), cookies: HashMap::new(), param: HashMap::new(), body: "".into() }
    }

    // /hello - !/hello -> None 
    #[test]
    fn blacklist() {
        let m = base("!/hello");
        let req = base_req("/hello");
        assert_eq!(None, m.verify(req));

    }


    // /hello - /hello -> Some 
    #[test]
    fn exact() {
        let m = base("/hello");
        let req = base_req("/hello");
        assert_eq!(Some(StatusCode::BadRequest), m.verify(req));
    }

    // /hello/hello - /hello -> None
    #[test]
    fn subpath_sended_too_much() {
        let m = base("/hello");
        let req = base_req("/hello/hello");
        assert_eq!(None, m.verify(req));
    }

    // /hello/hello - /hello/hello -> Some
    #[test]
    fn subpath_sended_exact() {
        let m = base("/hello/hello");
        let req = base_req("/hello/hello");
        assert_eq!(Some(StatusCode::BadRequest), m.verify(req));
    }

    // /hello/hello - /hello/* -> Some
    #[test]
    fn subpath_sended_subpath_allowed() {
        let m = base("/hello/*");
        let req = base_req("/hello/hello/");
        assert_eq!(Some(StatusCode::BadRequest), m.verify(req));
    }

    // /hello/hello - /hello/** -> Some
    #[test]
    fn subpath_sended_is_base() {
        let m = base("/hello/**");
        let req = base_req("/hello/hello");
        assert_eq!(Some(StatusCode::BadRequest), m.verify(req));
    }

    // /hello/hello/hello - /hello/* -> None
    #[test]
    fn subsubpath_sended_too_much() {
        let m = base("/hello/*");
        let req = base_req("/hello/hello/hello");
        assert_eq!(None, m.verify(req));
    }

    // /hello/hello/hello - /hello/** -> Some
    #[test]
    fn subsubpath_sended_base_allowed() {
        let m = base("/hello/**");
        let req = base_req("/hello/hello/hello");
        assert_eq!(Some(StatusCode::BadRequest), m.verify(req));
    }

}