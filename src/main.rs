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
    mprot.add("generate", |req: Value, state: CurrentState| {
        let body = req.get("body");
        json!({
            "src": state.node_id,
            "dest": req.get("src"),
            "body": {
                "type": "generate_ok",
                "msg_id": state.msg_id,
                "in_reply_to": body.and_then(|b| b.get("msg_id")),
                "id": state.generated_id
            }
        })
    });
    mprot.add("broadcast", |req: Value, state: CurrentState| {
        let body = req.get("body");
        json!({
            "src": state.node_id,
            "dest": req.get("src"),
            "body": {
                "type": "broadcast_ok",
                "msg_id": state.msg_id,
                "in_reply_to": body.and_then(|b| b.get("msg_id")),
            }

        })
    });
    mprot.add("read", |req: Value, state: CurrentState| {
        let body = req.get("body");
        eprintln!("{:?}",state.messages.clone());
        json!({
            "src": state.node_id,
            "dest": req.get("src"),
            "body": {
                "type": "read_ok",
                "messages": state.messages,
                "msg_id": state.msg_id,
                "in_reply_to": body.and_then(|b| b.get("msg_id")),
            }

        })
    });
    mprot.add("topology", |req: Value, state: CurrentState| {
        let body = req.get("body");
        json!({
            "src": state.node_id,
            "dest": req.get("src"),
            "body": {
                "type": "topology_ok",
                "msg_id": state.msg_id,
                "in_reply_to": body.and_then(|b| b.get("msg_id")),
            }

        })
    });
    mprot.add("add", |req: Value, state: CurrentState| {
        let body = req.get("body");
        unimplemented!()
    });


    mprot.run();
}
