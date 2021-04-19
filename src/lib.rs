use serde::{Deserialize, Serialize};

use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Opts {
    pub name: String,
    pub path: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BridgeResult {
    pub job_run_id: String,
    pub id: Option<String>,
    pub task_run_id: Option<String>,
    pub status: String,
    pub error: Option<String>,
    pub pending: bool,
    pub data: Value,
}

// Bridge is the trait that can be implemented for custom Chainlink bridges
pub trait Bridge {
    fn opts(&self) -> Opts;
    fn run(&self) -> (BridgeResult, Option<i64>);
}

// Server holds pointers to the bridges indexed by their paths
pub struct Server<'a> {
    pub path_map: HashMap<String, &'a dyn Bridge>,
    pub lda_bridge: dyn Bridge,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
