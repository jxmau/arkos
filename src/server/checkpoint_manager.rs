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

        for path in &self.checkpoint.except { // Iterate in all paths declared as exception - Return None as it is excempted if true.
            if compare(&path, &req.url) {
                return None;
            }
        }


        for path in &self.checkpoint.paths { // Iterate in all paths declared to be checked
            if compare(&path, &req.url) {
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

    // Exact pattern /
    if list.eq(req) {
        return true;
    }

    // Subpath allowed /* 
    else if list.ends_with("**") && req.starts_with(list.strip_suffix("**").unwrap()) {
        return true;
    }

    // Base is allowed /**
    else if list.ends_with("*") && req.starts_with(list.strip_suffix("*").unwrap()) {
        
        // list : /hello/*  req: /hello/hello
        if req.ends_with('/') {
            let req = req.strip_suffix('/').unwrap();
            return list.matches('/').count() == req.matches('/').count();
        }

    }

    false
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


    // Quick Word regarding those tests : We don't test the compare() function, 
    // but directly the verify method to be sure that exceptions are catched

    // Path that shan't be checked and returned early
    // We put the same path as path and except, for if the except fail, it will return a Some and the test can be qualified as false.

    // /hello - /hello -> None 
    #[test]
    fn except_exact() {
        let mut m = base("/hello");
        m.checkpoint.except("/hello");
        let req = base_req("/hello");
        assert_eq!(None, m.verify(req));
    }

    // /hello - /** but /hello exempted -> Some() as it doesn't match the exact pattern
    #[test]
    fn except_exact_incomplete_path() {
        let mut m = base("/**");
        m.checkpoint.except("/hello");
        let req = base_req("/hell");
        assert_eq!(Some(StatusCode::BadRequest), m.verify(req));
    }


    // Paths to be checked

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