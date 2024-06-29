use std::pin::Pin;

use crate::{
    compiled_protos::counter::{
        single_counter_server::SingleCounter, CounterDelta, CounterDeltaWithId, CounterState,
    },
    utils::RactorTonicErrorExt,
};
use actor::CounterActor;
use ractor::{Actor, ActorRef};
use tokio::{sync::broadcast, task::JoinHandle};
use tokio_stream::{
    wrappers::{errors::BroadcastStreamRecvError, BroadcastStream},
    Stream, StreamExt,
};
use tonic::{Request, Response, Status};

pub use crate::compiled_protos::counter::single_counter_server::SingleCounterServer;

mod actor;

pub struct SingleCounterService {
    actor: ActorRef<actor::Message>,
    event_tx: broadcast::Sender<actor::Event>,
}

impl SingleCounterService {
    pub async fn spawn(
        name: Option<String>,
    ) -> Result<(SingleCounterService, JoinHandle<()>), ractor::SpawnErr> {
        let (event_tx, _) = broadcast::channel(16);
        Actor::spawn(
            name,
            CounterActor {
                event_tx: event_tx.clone(),
            },
            (),
        )
        .await
        .map(|actor| {
            (
                Self {
                    actor: actor.0,
                    event_tx,
                },
                actor.1,
            )
        })
    }
}

impl Drop for SingleCounterService {
    fn drop(&mut self) {
        tracing::debug!("The single counter service is dropped.");
        self.actor.stop(None);
    }
}

#[tonic::async_trait]
impl SingleCounter for SingleCounterService {
    async fn increase(
        &self,
        request: Request<CounterDelta>,
    ) -> Result<Response<CounterState>, Status> {
        use actor::Message::Increase;

        let CounterDelta { delta } = request.into_inner();

        let counter = ractor::call!(self.actor, Increase, delta).map_err_internal()?;

        Ok(Response::new(counter.into()))
    }

    async fn current(&self, _request: Request<()>) -> Result<Response<CounterState>, Status> {
        use actor::Message::Retrieve;

        let counter = ractor::call!(self.actor, Retrieve).map_err_internal()?;

        Ok(Response::new(counter.into()))
    }

    type ListenDeltaStream = Pin<Box<dyn Stream<Item = Result<CounterDeltaWithId, Status>> + Send>>;

    async fn listen_delta(
        &self,
        _request: Request<()>,
    ) -> Result<Response<Self::ListenDeltaStream>, Status> {
        let rx = self.event_tx.subscribe();
        let rx = BroadcastStream::new(rx).filter_map(|retrieved| {
            use actor::Event::Update;

            match retrieved {
                Ok(Update { id, delta }) => Some(Ok(CounterDeltaWithId { delta, id })),
                Err(BroadcastStreamRecvError::Lagged(lag_num)) => {
                    Some(Err(Status::data_loss(format!(
                        "{} delta(s) have been lost since the receiver lagged too far behind.",
                        lag_num
                    ))))
                }
            }
        });

        Ok(Response::new(Box::pin(rx)))
    }
}
