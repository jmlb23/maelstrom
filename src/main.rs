use maelstrom::protocol::{CurrentState, Maelstrom};
use serde_json::{json, Value};

fn main() {
    let mut mprot = Maelstrom::new();
    mprot.add("init", |req: Value, state: CurrentState| {
        json!({
            "src": state.node_id,
            "dest": req.get("src"),
            "body": json!({
                "type": "init_ok",
                "msg_id": state.msg_id,
                "in_reply_to": req.get("body").and_then(|b| b.get("msg_id")),
            })
        })
    });
    mprot.add("echo", |req: Value, state: CurrentState| {
        let body = req.get("body");
        json!({
            "src": state.node_id,
            "dest": req.get("src"),
            "body": {
                "type": "echo_ok",
                "msg_id": state.msg_id,
                "in_reply_to": body.and_then(|b| b.get("msg_id")),
                "echo": body.and_then(|b| b.get("echo")),
            }
        })
    });
    mprot.run();
}
