use std::sync::Arc;

use crate::core::status::StatusCode;

use super::request::Request;


    // /    - Explicitly one path
    // /*   - Any subpath of this
    // /**  - Start with this
    // !/   - Doesn't apply to this path


/// Struct to filter request before responding.
#[derive(Clone)]
pub struct Checkpoint {
    #[doc(hidden)]
    pub paths: Vec<String>,
    #[doc(hidden)]
    pub check: Arc<dyn Fn(Request) -> Result<(), StatusCode> + Send + Sync>
}

impl Checkpoint {

    /// Create a new Checkpoint.
    pub fn new(paths: Vec<String>, check: Arc< dyn Fn(Request) -> Result<(), StatusCode> + Send + Sync> ) -> Self {
        Checkpoint { paths, check}
    }

}