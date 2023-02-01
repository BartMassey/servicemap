pub mod services;

pub use services::*;

use std::any::Any;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceMapError {
    #[error("incorrect service API type")]
    IncorrectAPIType,
    #[error("invalid service key")]
    InvalidKey,
}

pub type ServiceResult<T> = Result<T, ServiceMapError>;

pub trait ServicePackage {
    fn package() -> Box<dyn Any + 'static>;
}

#[derive(Default)]
pub struct ServiceMap(Vec<Box<dyn Any + 'static>>);

impl ServiceMap {
    pub fn register<P: ServicePackage>(&mut self) -> usize {
        let key = self.0.len();
        self.0.push(P::package());
        key
    }
}
