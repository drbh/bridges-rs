use bridges_rs::*;
use isahc::prelude::*;
use rouille::router;
use serde_json::Value;
use std::io::Read;

struct GasStation {}

impl Bridge for GasStation {
    fn opts(&self) -> Opts {
        Opts {
            name: String::from("GasStation"),
            path: None,
        }
    }
    fn run(&self) -> (BridgeResult, Option<i64>) {
        let mut response = isahc::get("https://ethgasstation.info/json/ethgasAPI.json").unwrap();
        let value: Value = serde_json::from_str(&response.text().unwrap()).unwrap();
        let br = BridgeResult {
            job_run_id: String::from("test"),
            id: None,
            task_run_id: None,
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

    // should wrap this into the lib
    println!("Now listening on localhost:8081");
    rouille::start_server("localhost:8081", move |request| {
        router!(request,
            (POST) (/) => {
                println!("{:#?}", request);

                let mut data = request.data().expect("Oops, body already retrieved, problem \
                                                      in the server");
                let mut buf = Vec::new();
                match data.read_to_end(&mut buf) {
                    Ok(_) => (),
                    Err(_) => ()
                };
                let s = match std::str::from_utf8(&buf) {
                    Ok(v) => v,
                    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                };

                let value: Value = serde_json::from_str(&s).unwrap();
                println!("result: {}", value);

                let cc = GasStation {};
                let (resp, _) = cc.run();
                rouille::Response::json(&resp)
            },
            _ => rouille::Response::empty_404()
        )
    });
}
