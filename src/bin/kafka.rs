use std::collections::HashMap;

use anyhow::Context;
use maelstrom::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Payload {
    Send {
        key: String,
        msg: usize,
    },
    SendOk {
        offset: usize,
    },
    Poll {
        offsets: HashMap<String, usize>,
    },
    PollOk {
        msgs: HashMap<String, Vec<(usize, usize)>>,
    },
    CommitOffsets {
        offsets: HashMap<String, usize>,
    },
    CommitOffsetsOk,
    ListCommittedOffsets {
        keys: Vec<String>,
    },
    ListCommittedOffsetsOk {
        offsets: HashMap<String, usize>,
    },
}
enum InjectedPayload {}

struct KafkaNode {
    id: usize,
    data: HashMap<String, Vec<usize>>,
    committed_offsets: HashMap<String, usize>,
}

impl Node<(), Payload, InjectedPayload> for KafkaNode {
    fn from_init(
        _state: (),
        _init: Init,
        _tx: std::sync::mpsc::Sender<Event<Payload, InjectedPayload>>,
    ) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        // std::thread::spawn(move || {
        //     loop {
        //         std::thread::sleep(Duration::from_millis(20));
        //         if let Err(_) = tx.send(Event::Injected(InjectedPayload::UppdateValue)) {
        //             break;
        //         }
        //     }
        // });

        Ok(KafkaNode {
            id: 0,
            data: HashMap::new(),
            committed_offsets: HashMap::new(),
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
                // InjectedPayload::UppdateValue => {
                //     for n in &self.nodes {
                //         let latest_value = self.latest_value;
                //         let message = Message::new(
                //             self.node.clone(),
                //             n.clone(),
                //             &mut self.id,
                //             Payload::Uppdate { latest_value },
                //         );
                //         message.send(output).context("send uppdate")?;
                //     }
                // }
            },
            Event::Message(msg) => {
                let mut reply: Message<Payload> = msg.into_reply(Some(&mut self.id));
                match reply.body.payload {
                    Payload::Send { key, msg } => {
                        let entry = self.data.entry(key.clone()).or_default();
                        entry.push(msg);
                        let offset = entry.len() - 1;
                        reply.body.payload = Payload::SendOk { offset };
                        reply.send(output).context("send response to send")?;
                    }

                    Payload::Poll { offsets } => {
                        let mut msgs: HashMap<String, Vec<(usize, usize)>> = HashMap::new();
                        for (key, offset) in offsets {
                            if let Some(entries) = self.data.get(&key) {
                                let polled: Vec<(usize, usize)> = entries
                                    .iter()
                                    .enumerate()
                                    .skip(offset)
                                    .map(|(i, &msg)| (i, msg))
                                    .collect();
                                msgs.insert(key.clone(), polled);
                            } else {
                                msgs.insert(key.clone(), Vec::new());
                            }
                        }
                        reply.body.payload = Payload::PollOk { msgs };
                        reply.send(output).context("send response to poll")?;
                    }
                    Payload::CommitOffsets { offsets } => {
                        for (key, offset) in offsets {
                            self.committed_offsets
                                .entry(key)
                                .and_modify(|curr| *curr = (*curr).max(offset))
                                .or_insert(offset);
                        }
                        reply.body.payload = Payload::CommitOffsetsOk;
                        reply
                            .send(output)
                            .context("send response to commit offsets")?;
                    }
                    Payload::ListCommittedOffsets { keys } => {
                        let mut offsets: HashMap<String, usize> = HashMap::new();
                        for key in keys {
                            if let Some(&val) = self.committed_offsets.get(&key) {
                                offsets.insert(key, val);
                            }
                        }
                        reply.body.payload = Payload::ListCommittedOffsetsOk { offsets };
                        reply
                            .send(output)
                            .context("send response to list committed offsets")?;
                    }

                    Payload::SendOk { .. }
                    | Payload::PollOk { .. }
                    | Payload::CommitOffsetsOk
                    | Payload::ListCommittedOffsetsOk { .. } => {
                        // ignore
                    }
                }
            }
        }
        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    main_loop::<_, KafkaNode, _, _>(())
}
