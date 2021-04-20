use bridges_rs::*;
use isahc::prelude::*;
use serde_json::Value;

struct GasStation {}

impl Bridge for GasStation {
    fn opts(&self) -> Opts {
        Opts {
            name: String::from("GasStation"),
            path: None,
        }
    }
    fn run(&self, job_id: String) -> (BridgeResult, Option<i64>) {
        let mut response = isahc::get("https://ethgasstation.info/json/ethgasAPI.json").unwrap();
        let value: Value = serde_json::from_str(&response.text().unwrap()).unwrap();
        let br = BridgeResult {
            job_run_id: job_id,
            // id: None,
            // task_run_id: None,
            status: String::from("completed"),
            error: None,
            pending: false,
            data: value,
        };
        (br, None)
    }
}

fn main() {
    //
    let gs = GasStation {};
    let s = Server::new(gs);
    s.start_server();
}
