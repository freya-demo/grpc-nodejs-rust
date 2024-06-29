use std::ops::{Bound, RangeBounds};

use ractor::{Actor, ActorProcessingErr, ActorRef, MessagingErr, RpcReplyPort};
use tokio::sync::broadcast;

pub struct CounterActor {
    pub event_tx: broadcast::Sender<Event>,
}

#[derive(Clone)]
pub enum Event {
    Update { id: u32, delta: i32 },
}

pub struct State {
    counter: i32,
    deltas: Vec<i32>,
}

pub enum Message {
    Increase(i32, RpcReplyPort<i32>),
    Retrieve(RpcReplyPort<i32>),
    GetDeltas((Bound<usize>, Bound<usize>), RpcReplyPort<Vec<i32>>),
}

impl State {
    fn increase(&mut self, actor: &CounterActor, delta: i32) -> i32 {
        self.counter += delta;
        self.deltas.push(delta);
        let _ = actor.event_tx.send(Event::Update {
            id: (self.deltas.len() - 1) as u32,
            delta,
        });
        self.counter
    }

    fn retrieve(&mut self) -> i32 {
        self.counter
    }

    fn get_deltas(&mut self, range: (Bound<usize>, Bound<usize>)) -> Vec<i32> {
        self.deltas[range].to_vec()
    }
}

#[async_trait::async_trait]
impl Actor for CounterActor {
    type Msg = Message;
    type State = State;
    type Arguments = ();

    async fn pre_start(
        &self,
        _myself: ActorRef<Self::Msg>,
        _: (),
    ) -> Result<Self::State, ActorProcessingErr> {
        Ok(State {
            counter: 0,
            deltas: Vec::new(),
        })
    }

    async fn handle(
        &self,
        _myself: ActorRef<Self::Msg>,
        message: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        fn log_err<T>(err: MessagingErr<T>) {
            tracing::warn!(?err, "failed to send the counter back.")
        }

        match message {
            Message::Increase(delta, reply) => {
                if let Err(err) = reply.send(state.increase(&self, delta)) {
                    log_err(err)
                }
            }
            Message::Retrieve(reply) => {
                if let Err(err) = reply.send(state.retrieve()) {
                    log_err(err)
                }
            }
            Message::GetDeltas(range, reply) => {
                if let Err(err) = reply.send(state.get_deltas(range)) {
                    log_err(err)
                }
            }
        };

        Ok(())
    }
}
