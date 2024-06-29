use ractor::RactorErr;
use tonic::Status;

pub trait RactorTonicErrorExt<T> {
    fn map_err_internal(self) -> Result<T, Status>;
}

impl<T, M> RactorTonicErrorExt<T> for Result<T, RactorErr<M>> {
    fn map_err_internal(self) -> Result<T, Status> {
        self.map_err(|err| Status::internal(err.to_string()))
    }
}
