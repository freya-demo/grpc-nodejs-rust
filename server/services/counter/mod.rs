use crate::{
    compiled_protos::counter::{
        single_counter_server::SingleCounter, CounterIncreaseRequest, CounterState,
    },
    utils::RactorTonicErrorExt,
};
use actor::CounterActor;
use ractor::{Actor, ActorRef};
use tokio::task::JoinHandle;
use tonic::{Request, Response, Status};

pub use crate::compiled_protos::counter::single_counter_server::SingleCounterServer;

mod actor;

pub struct SingleCounterService(ActorRef<actor::Message>);

impl SingleCounterService {
    pub async fn spawn(
        name: Option<String>,
    ) -> Result<(SingleCounterService, JoinHandle<()>), ractor::SpawnErr> {
        Actor::spawn(name, CounterActor, ())
            .await
            .map(|actor| (Self(actor.0), actor.1))
    }
}

impl Drop for SingleCounterService {
    fn drop(&mut self) {
        tracing::debug!("The single counter service is dropped.");
        self.0.stop(None);
    }
}

#[tonic::async_trait]
impl SingleCounter for SingleCounterService {
    async fn increase(
        &self,
        request: Request<CounterIncreaseRequest>,
    ) -> Result<Response<CounterState>, Status> {
        use actor::Message::Increase;

        let CounterIncreaseRequest { delta } = request.into_inner();

        let counter = ractor::call!(self.0, Increase, delta).map_err_internal()?;

        Ok(Response::new(counter.into()))
    }

    async fn current(&self, _request: Request<()>) -> Result<Response<CounterState>, Status> {
        use actor::Message::Retrieve;

        let counter = ractor::call!(self.0, Retrieve).map_err_internal()?;

        Ok(Response::new(counter.into()))
    }
}
