use ractor::{Actor, ActorProcessingErr, ActorRef, MessagingErr, RpcReplyPort};

pub struct CounterActor;

pub struct State {
    counter: i32,
}

pub enum Message {
    Increase(i32, RpcReplyPort<i32>),
    Retrieve(RpcReplyPort<i32>),
}

impl State {
    fn increase(&mut self, delta: i32) -> i32 {
        self.counter += delta;
        self.counter
    }

    fn retrieve(&mut self) -> i32 {
        self.counter
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
        Ok(State { counter: 0 })
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
                if let Err(err) = reply.send(state.increase(delta)) {
                    log_err(err)
                }
            }
            Message::Retrieve(reply) => if let Err(err) = reply.send(state.retrieve()) {
                log_err(err)
            },
        };

        Ok(())
    }
}
