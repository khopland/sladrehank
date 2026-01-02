use anyhow::Context;
use maelstrom_echo::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Payload {
    Generate,
    GenerateOk {
        #[serde(rename = "id")]
        guid: String,
    },
}
struct UniqueNode {
    node:String,
    id: usize,
}
impl Node<(), Payload> for UniqueNode {
    fn from_init(
        _state: (),
        init: Init,
        _inject: std::sync::mpsc::Sender<Event<Payload, ()>>,
    ) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(UniqueNode {
            node: init.node_id,
            id: 1
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
                    Payload::Generate => {
                        let guid = format!("{}-{}", self.node, self.id);
                        reply.body.payload = Payload::GenerateOk { guid };
                        reply.send(output).context("send echo Ok")?;
                    }
                    Payload::GenerateOk { .. } => {}
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
    main_loop::<_, UniqueNode, _, _>(())
}
