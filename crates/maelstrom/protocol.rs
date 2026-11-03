use std::collections::{HashMap, HashSet};

use serde_json::{json, Value};
use uuid::Uuid;

#[derive(Default)]
pub struct CurrentState {
    pub msg_id: u64,
    pub node_id: String,
    pub generated_id: String,
    pub messages: HashSet<i64>,
}

#[derive(Default)]
pub struct Maelstrom {
    next_msg_id: u64,
    node_id: String,
    messages: HashSet<i64>,
    topology: HashSet<String>,
    callbacks: HashMap<String, Box<dyn Fn(Value, CurrentState) -> Value + 'static>>,
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

    fn send(origin: String, destination: String, message: i64) {
        let payload = json!({
            "src": origin,
            "dest": destination,
            "body": {
                "type": "broadcast",
                "message": message
            }
        });
        println!("{}", payload)
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
                Some("echo") => {
                    self.callbacks.get("echo").and_then(|cal| {
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
                Some("generate") => {
                    self.callbacks.get("generate").and_then(|cal| {
                        value.map(|pr| {
                            println!(
                                "{}",
                                cal(
                                    pr,
                                    CurrentState {
                                        node_id: self.node_id.to_string(),
                                        msg_id: self.next_msg_id,
                                        generated_id: format!("{}", Uuid::new_v4().to_string()),
                                        ..Default::default()
                                    }
                                )
                            )
                        })
                    });
                }
                Some("broadcast") => {
                    value
                        .as_ref()
                        .and_then(|v| v.get("body"))
                        .and_then(|b| b.get("message"))
                        .and_then(|m| m.as_i64())
                        .map(|msg| {
                            let ins_res = self.messages.insert(msg);
                            if ins_res {
                                for neighbor in self.topology.clone() {
                                    Maelstrom::send(self.node_id.clone(), neighbor, msg.clone());
                                }
                            }
                        });
                    self.callbacks.get("broadcast").and_then(|cal| {
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
                                        messages: self.messages.clone(),
                                        ..Default::default()
                                    }
                                )
                            )
                        })
                    });
                }
                Some("topology") => {
                    let self_id = self.node_id.clone();
                    let topology = value
                        .as_ref()
                        .and_then(|v| v.get("body"))
                        .and_then(|b| b.get("topology").map(|t| t.as_object()))
                        .and_then(|t| t.and_then(|m| m.get(self_id.as_str())))
                        .and_then(|ta| {
                            ta.as_array().map(|a| {
                                a.iter()
                                    .flat_map(|it| it.as_str())
                                    .map(|it| it.to_string())
                                    .collect::<HashSet<String>>()
                            })
                        })
                        .unwrap_or_default();
                    self.topology = topology;

                    self.callbacks.get("topology").and_then(|cal| {
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
                Some("add") => {
                    unimplemented!()
                }
                Some("broadcast_ok") => {
                    
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
