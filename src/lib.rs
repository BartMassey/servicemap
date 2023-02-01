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

#[derive(Default)]
pub struct ServiceMap(Vec<Box<dyn Any + 'static>>);

impl ServiceMap {
    pub fn register(&mut self, package: Box<dyn Any + 'static>) -> usize {
        let key = self.0.len();
        self.0.push(package);
        key
    }

    pub(crate) fn get_service<T: 'static>(&mut self, key: usize) -> ServiceResult<&mut T> {
        let entry = self.0.get_mut(key).ok_or(ServiceMapError::InvalidKey)?;
        entry
            .downcast_mut::<T>()
            .ok_or(ServiceMapError::IncorrectAPIType)
    }
}
