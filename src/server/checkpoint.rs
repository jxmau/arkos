use std::sync::Arc;

use crate::core::status::StatusCode;

use super::request::Request;

/// 
/// Path declared | Path requested | Mathes
/// ---- | ------ | :---: |
/// /hello | /hello | Yes
/// /hello | /hello/hello | No
/// /hello | /hello/hello/hello | No
/// /hello/* | /hello | No
/// /hello/* | /hello/hello | Yes
/// /hello/* | /hello/hello/hello | No
/// /hello/** | /hello | No
/// /hello/** | /hello/hello | Yes
/// /hello/** | /hello/hello/hello | Yes
///
///
/// Struct to filter request before responding.
#[derive(Clone)]
pub struct Checkpoint {
    pub(crate) paths: Vec<String>,
    pub(crate) except: Vec<String>,
    pub(crate) check: Arc<dyn Fn(Request) -> Result<(), StatusCode> + Send + Sync>
}

impl Checkpoint {

    /// Create a new Checkpoint.
    pub fn new(check: Arc< dyn Fn(Request) -> Result<(), StatusCode> + Send + Sync> ) -> Self {
        Checkpoint {paths: Vec::new(), check, except: Vec::new()}
    }

    /// Add a path to the list of checked paths.
    pub fn on(&mut self, path: &str) {
        self.paths.push(path.into());
    }
    
    /// Add several paths to the list of checked paths.
    pub fn on_all(&mut self, paths: Vec<&str>) {
        for p in paths {
            self.paths.push(p.into());
        }
    }
    
    /// Will add a list of to the paths list, replacing the paths previously added.
    /// If you need to add oly one path, use the method `on`
    pub fn set_paths(&mut self, paths: Vec<&str>) {
        let mut v : Vec<String> = Vec::new();
        for p in paths {
            v.push(p.into())
        }
        self.paths = v;

    }   
    
    /// Add a path to the exception list.
    pub fn except(&mut self, path: &str) {
        self.except.push(path.into())
    }
    
    /// Add several paths to the Exception list.
    pub fn except_all(&mut self, paths: Vec<&str>) {
        for p in paths {
            self.except.push(p.into());
        }
    }
    
    /// Will add a list of to the paths list, replacing the paths previously added.
    /// If you need to add oly one path, use the method `on`
    pub fn set_exception_paths(&mut self, paths: Vec<&str>) {
        let mut v : Vec<String> = Vec::new();
        for p in paths {
            v.push(p.into())
        }
        self.except = v;
    
    }
    
}