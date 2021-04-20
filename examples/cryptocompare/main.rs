use bridges_rs::*;
use isahc::prelude::*;
use rouille::router;
use serde_json::Value;
use std::io::Read;

struct CryptoCompare {}

impl Bridge for CryptoCompare {
    fn opts(&self) -> Opts {
        Opts {
            name: String::from("CryptoCompare"),
            path: None,
        }
    }

    fn run(&self, job_id: String) -> (BridgeResult, Option<i64>) {
        let mut response =
            isahc::get("https://min-api.cryptocompare.com/data/price?fsym=ETH&tsyms=USD,JPY,EUR")
                .unwrap();
        let value: Value = serde_json::from_str(&response.text().unwrap()).unwrap();
        let br = BridgeResult {
            job_run_id: String::from("test"),
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
    let cc = CryptoCompare {};
    let s = Server::new(cc);
    s.start_server();
}
