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
        let bridge_repsonse = BridgeResult {
            job_run_id: job_id,
            status: String::from("completed"),
            error: None,
            pending: false,
            data: value,
        };
        (bridge_repsonse, None)
    }
}

fn main() {
    //
    let gs = GasStation {};
    let s = Server::new(gs);
    s.start_server();
}
