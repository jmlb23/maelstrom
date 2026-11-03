use maelstrom::protocol::{CurrentState, Maelstrom};
use serde_json::{json, Value};

fn main() {
    let mut mprot = Maelstrom::new();
    mprot.add("init", |req: Value, state: CurrentState| {
        json!({
            "src": state.node_id,
            "dest": req.get("src"),
            "body": {
                "type": "init_ok",
                "msg_id": state.msg_id,
                "in_reply_to": req.get("body").and_then(|b| b.get("msg_id")),
            }
        })
    });
    mprot.add("read", |req: Value, state: CurrentState| {
        let body = req.get("body");
        json!({
            "src": state.node_id,
            "dest": req.get("src"),
            "body": {
                "type": "read_ok",
                "value": state.g_set,
                "msg_id": state.msg_id,
                "in_reply_to": body.and_then(|b| b.get("msg_id")),
            }

        })
    });
    mprot.add("add", |req: Value, state: CurrentState| {
        let body = req.get("body");
        json!({
            "src": state.node_id,
            "dest": req.get("src"),
            "body": {
                "type": "add_ok",
                "msg_id": state.msg_id,
                "in_reply_to": body.and_then(|b| b.get("msg_id")),
            }

        })
    });

    mprot.run();
}
