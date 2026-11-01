use std::collections::HashMap;

use serde_json::Value;

pub struct CurrentState {
    pub msg_id: u64,
    pub node_id: String,
}

pub struct Maelstrom {
    next_msg_id: u64,
    node_id: String,
    callbacks: HashMap<String, Box<dyn Fn(Value, CurrentState) -> Value + 'static>>,
}

impl Maelstrom {
    pub fn new() -> Self {
        Maelstrom {
            next_msg_id: 0,
            node_id: String::new(),
            callbacks: HashMap::new(),
        }
    }

    pub fn add(&mut self, cmd: &str, callback: impl Fn(Value, CurrentState) -> Value + 'static) {
        self.callbacks.insert(cmd.to_string(), Box::new(callback));
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
                .clone()
                .as_ref()
                .and_then(|v| v.get("body"))
                .and_then(|b| b.get("type"))
                .and_then(|t| t.as_str())
            {
                Some("init") => {
                    let node_id = value
                        .clone()
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
                                        msg_id: self.next_msg_id
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
                                        msg_id: self.next_msg_id
                                    }
                                )
                            )
                        })
                    });
                }
                _ => {
                    panic!("something weird happened")
                }
            }
            buf.clear();
        }
    }
}
