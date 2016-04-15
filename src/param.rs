//! Parameters for talking to query-based AWS services.
//!
//! Key-value pairs for AWS query requests.
//!
//! Supports optional parameters for calling SQS and ETS.

use std::collections::BTreeMap;
pub type Params = BTreeMap<String, String>;

/// Key:value pair for an service parameter.
pub trait ServiceParams {
    fn put(&mut self, key: &str, val: &str);
}

impl ServiceParams for Params {
    fn put(&mut self, key: &str, val: &str) {
        self.insert(key.into(), val.into());
    }
}
