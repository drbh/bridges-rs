use rouille::router;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Debug;
use std::io::Read;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Debug)]
pub struct Opts {
    pub name: String,
    pub path: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestInput {
    pub job_run_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BridgeResult {
    pub job_run_id: String,
    // pub id: Option<String>,
    // pub task_run_id: Option<String>,
    pub status: String,
    pub error: Option<String>,
    pub pending: bool,
    pub data: Value,
}

// Bridge is the trait that can be implemented for custom Chainlink bridges
pub trait Bridge: Sync + Send {
    fn opts(&self) -> Opts;
    fn run(&self, job_id: String) -> (BridgeResult, Option<i64>);
}

impl Debug for dyn Bridge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Bridge").finish()
    }
}

// Server holds pointers to the bridges indexed by their paths
pub struct Server {
    // pub path_map: HashMap<String, Arc<Mutex<Box<dyn Bridge>>>>,
    pub bridge: Arc<Mutex<Box<dyn Bridge>>>,
}

impl Server {
    pub fn new(bridge: impl Bridge + 'static) -> Self {
        // let mut hm: HashMap<String, Box<dyn Bridge>> = HashMap::new();
        // self.path_map = hm;
        Self {
            bridge: Arc::new(Mutex::new(Box::new(bridge))),
        }
    }

    pub fn start_server(self) {
        println!("Now listening on localhost:8081");
        rouille::start_server("localhost:8081", move |request| {
            router!(request,
                (POST) (/) => {
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
                    let request_input: RequestInput = serde_json::from_str(&s).unwrap();

                    let cc = self.bridge.lock().unwrap();


                    let job_id = match request_input.job_run_id {
                        Some(jid) => String::from(jid),
                        None => String::from("randomId")
                    };

                    let (resp, _) = cc.run(job_id);
                    rouille::Response::json(&resp)
                },
                _ => rouille::Response::empty_404()
            )
        });
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
