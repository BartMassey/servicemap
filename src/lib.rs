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
    #[error("invalid ervice key")]
    InvalidKey,
}

pub type ServiceResult<T> = Result<T, ServiceMapError>;

pub trait ServicePackage {
    fn package() -> Box<dyn Service>;
}

pub trait Service {
    fn call(&mut self, args: &(dyn Any + 'static)) -> ServiceResult<Box<dyn Any + 'static>>;
}

#[derive(Default)]
pub struct ServiceMap(Vec<Box<dyn Service>>);

impl ServiceMap {
    pub fn register<P: ServicePackage>(&mut self) -> usize {
        let key = self.0.len();
        self.0.push(P::package());
        key
    }
}
