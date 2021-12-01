use std::sync::Arc;

use crate::core::status::StatusCode;

use super::request::Request;


    // /    - Explicitly one path
    // /*   - Any subpath of this
    // /**  - Start with this


/// Struct to filter request before responding.
#[derive(Clone)]
pub struct Checkpoint {
    #[doc(hidden)]
    pub paths: Vec<String>,
    #[doc(hidden)]
    pub except: Vec<String>,
    #[doc(hidden)]
    pub check: Arc<dyn Fn(Request) -> Result<(), StatusCode> + Send + Sync>
}

impl Checkpoint {

    /// Create a new Checkpoint.
    pub fn new(paths: Vec<String>, check: Arc< dyn Fn(Request) -> Result<(), StatusCode> + Send + Sync> ) -> Self {
        Checkpoint { paths, check, except: Vec::new()}
    }

    /// Add a path to the exception list.
    pub fn except(&mut self, path: &str) {
        self.except.push(path.into())
    }

    /// Replace the exception list with the Vec provided as param.
    pub fn except_all(&mut self, paths: Vec<String>) {
        self.except = paths;
    }

}