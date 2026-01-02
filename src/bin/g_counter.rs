use std::{collections::HashMap, time::Duration};

use anyhow::Context;
use maelstrom::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Payload {
    Add { delta: usize },
    AddOk,
    Uppdate { latest_value: usize },
    Read,
    ReadOk { value: usize },
}
enum InjectedPayload {
    UppdateValue,
}

struct CounterNode {
    node: String,
    id: usize,
    latest_value: usize,
    other_values: HashMap<String, usize>,
    nodes: Vec<String>,
}

impl Node<(), Payload, InjectedPayload> for CounterNode {
    fn from_init(
        _state: (),
        init: Init,
        tx: std::sync::mpsc::Sender<Event<Payload, InjectedPayload>>,
    ) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        std::thread::spawn(move || {
            loop {
                std::thread::sleep(Duration::from_millis(20));
                if let Err(_) = tx.send(Event::Injected(InjectedPayload::UppdateValue)) {
                    break;
                }
            }
        });

        Ok(CounterNode {
            node: init.node_id.clone(),
            id: 0,
            latest_value: 0,
            other_values: HashMap::new(),
            nodes: init
                .node_ids
                .into_iter()
                .filter(|n| n != &init.node_id)
                .collect(),
        })
    }

    fn step(
        &mut self,
        input: Event<Payload, InjectedPayload>,
        output: &mut std::io::StdoutLock,
    ) -> anyhow::Result<()> {
        match input {
            Event::EOF => {
                // do nothing
            }
            Event::Injected(injected) => match injected {
                InjectedPayload::UppdateValue => {
                    for n in &self.nodes {
                        let latest_value = self.latest_value;
                        let message = Message::new(
                            self.node.clone(),
                            n.clone(),
                            &mut self.id,
                            Payload::Uppdate { latest_value },
                        );
                        message.send(output).context("send uppdate")?;
                    }
                }
            },
            Event::Message(msg) => {
                let src = msg.src.clone();
                let mut reply: Message<Payload> = msg.into_reply(Some(&mut self.id));
                match reply.body.payload {
                    Payload::Add { delta } => {
                        self.latest_value += delta;
                        reply.body.payload = Payload::AddOk;
                        reply.send(output).context("send add ok")?;
                    }
                    Payload::Uppdate { latest_value } => {
                        self.other_values.insert(src, latest_value);
                    }
                    Payload::Read => {
                        let mut value = self.latest_value;
                        value += self.other_values.values().sum::<usize>();
                        reply.body.payload = Payload::ReadOk { value };
                        reply.send(output).context("send read ok")?;
                    }
                    Payload::ReadOk { .. } | Payload::AddOk => {}
                }
            }
        }
        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    main_loop::<_, CounterNode, _, _>(())
}
