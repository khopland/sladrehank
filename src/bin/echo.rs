use anyhow::Context;
use maelstrom::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Payload {
    Echo { echo: String },
    EchoOk { echo: String },
}
struct EchoNode {
    id: usize,
}
impl Node<(), Payload> for EchoNode {
    fn from_init(
        _state: (),
        init: Init,
        _inject: std::sync::mpsc::Sender<Event<Payload, ()>>,
    ) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(EchoNode {
            id: init.node_id.parse().unwrap_or(0),
        })
    }

    fn step(
        &mut self,
        input: Event<Payload, ()>,
        output: &mut std::io::StdoutLock,
    ) -> anyhow::Result<()> {
        match input {
            Event::Message(msg) => {
                let mut reply = msg.into_reply(Some(&mut self.id));
                match reply.body.payload {
                    Payload::Echo { echo } => {
                        reply.body.payload = Payload::EchoOk { echo };
                        reply.send(&mut *output).context("send echo Ok")?;
                    }
                    Payload::EchoOk { .. } => {}
                }
            }
            _ => {
                panic!("unexpected event");
            }
        }
        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    main_loop::<_, EchoNode, _, _>(())
}
