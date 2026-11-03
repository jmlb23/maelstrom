use std::collections::{HashMap, HashSet};

use serde_json::{json, Value};

#[derive(Default, Clone)]
pub struct CurrentState {
    pub msg_id: u64,
    pub node_id: String,
    pub generated_id: String,
    pub g_set: HashSet<u64>,
}

#[derive(Default)]
pub struct Maelstrom {
    next_msg_id: u64,
    node_id: String,
    topology: HashSet<String>,
    callbacks: HashMap<String, Box<dyn Fn(Value, CurrentState) -> Value + 'static>>,
    g_set: HashSet<u64>,
}

impl Maelstrom {
    pub fn new() -> Self {
        Maelstrom {
            ..Default::default()
        }
    }

    pub fn add(&mut self, cmd: &str, callback: impl Fn(Value, CurrentState) -> Value + 'static) {
        self.callbacks.insert(cmd.to_string(), Box::new(callback));
    }

    fn extract_topology(value: Option<Value>) -> HashSet<String> {
        let body = value.as_ref().and_then(|v| v.get("body"));
        let node_ids = body
            .and_then(|b| b.get("node_ids"))
            .and_then(|nid| nid.as_array())
            .map(|nids| {
                nids.iter()
                    .filter_map(|str| str.as_str().map(|str| str.to_string()))
                    .collect::<HashSet<String>>()
            })
            .unwrap_or_default();

        node_ids
    }

    fn replicate(origin: String, destinations: HashSet<String>, set: HashSet<u64>) {
        for dest in destinations {
            if dest == origin {
                continue;
            }
            let response = json!({
                "src": origin,
                "dest": dest,
                "body": {
                    "type": "replicate",
                    "value": set
                }
            });
            eprintln!("{}", response.clone());
            println!("{}", response)
        }
    }

    pub fn run(&mut self) {
        loop {
            let mut buf = String::new();
            let _ = std::io::stdin().read_line(&mut buf);
            let value = serde_json::from_str::<Value>(buf.as_str()).ok();
            if buf.trim() == "quit" {
                eprintln!("quitting gracefuly");
                break;
            }
            self.next_msg_id += 1;
            match value
                .as_ref()
                .and_then(|v| v.get("body"))
                .and_then(|b| b.get("type"))
                .and_then(|t| t.as_str())
            {
                Some("init") => {
                    let node_id = value
                        .as_ref()
                        .and_then(|v| v.get("body"))
                        .and_then(|b| b.get("node_id"))
                        .and_then(|n| n.as_str())
                        .map(|st| st.to_string())
                        .unwrap_or_default();

                    self.node_id = node_id.clone();
                    self.topology = Maelstrom::extract_topology(value.clone());
                    self.callbacks.get("init").and_then(|cal| {
                        value.map(|pr| {
                            println!(
                                "{}",
                                cal(
                                    pr,
                                    CurrentState {
                                        node_id: node_id.clone(),
                                        msg_id: self.next_msg_id,
                                        ..Default::default()
                                    }
                                )
                            )
                        })
                    });
                }
                Some("read") => {
                    self.callbacks.get("read").and_then(|cal| {
                        value.map(|pr| {
                            println!(
                                "{}",
                                cal(
                                    pr,
                                    CurrentState {
                                        node_id: self.node_id.to_string(),
                                        msg_id: self.next_msg_id,
                                        g_set: self.g_set.clone(),
                                        ..Default::default()
                                    }
                                )
                            )
                        })
                    });
                }
                Some("add") => {
                    let element = value
                        .as_ref()
                        .and_then(|v| v.get("body"))
                        .and_then(|b| b.get("element"))
                        .and_then(|e| e.as_u64());
                    if let Some(ele) = element {
                        if self.g_set.insert(ele) {
                            Maelstrom::replicate(
                                self.node_id.clone(),
                                self.topology.clone(),
                                self.g_set.clone(),
                            );
                        }
                    }
                    self.callbacks.get("add").and_then(|cal| {
                        value.map(|pr| {
                            println!(
                                "{}",
                                cal(
                                    pr,
                                    CurrentState {
                                        node_id: self.node_id.to_string(),
                                        msg_id: self.next_msg_id,
                                        ..Default::default()
                                    }
                                )
                            )
                        })
                    });
                }
                Some("replicate") => {
                    let body = value.as_ref().and_then(|v| v.get("body"));
                    let replicate = body
                        .and_then(|b| b.get("value"))
                        .and_then(|val| val.as_array())
                        .map(|vals| {
                            vals.iter()
                                .filter_map(|u| u.as_u64())
                                .collect::<HashSet<u64>>()
                        })
                        .unwrap_or_default();
                    self.g_set = self.g_set.union(&replicate).cloned().collect();
                }
                _ => {
                    panic!(
                        "something weird happened {:?}",
                        value.as_ref().and_then(|v| v.get("body"))
                    )
                }
            }
            buf.clear();
        }
    }
}
