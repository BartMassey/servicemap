pub mod services;

pub use services::*;

use std::any::Any;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceMapError {
    #[error("incorrect service argument type")]
    IncorrectArgumentType,
    #[error("incorrect service result type")]
    IncorrectResultType,
}

pub type ServiceResult<T> = Result<T, ServiceMapError>;

pub trait ServicePackage {
    type CallArgs: 'static;
    type CallResult: 'static;

    fn package() -> ServiceEntry;
}

pub(crate) trait Service {
    fn call(&mut self, args: &dyn Any) -> ServiceResult<Box<dyn Any>>;
}

pub struct ServiceEntry(Box<dyn Service>);

#[derive(Default)]
pub struct ServiceMap(Vec<ServiceEntry>);

impl ServiceMap {
    pub fn register<P: ServicePackage>(&mut self) -> usize {
        let key = self.0.len();
        self.0.push(P::package());
        key
    }

    pub fn invoke<P: ServicePackage>(
        &mut self,
        key: usize,
        args: P::CallArgs,
    ) -> ServiceResult<Box<P::CallResult>> {
        let result = self.0[key].0.call(&args)?;
        result.downcast::<P::CallResult>().map_err(|_| ServiceMapError::IncorrectResultType)
    }
}
