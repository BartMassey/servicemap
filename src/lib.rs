//! This "service map" allows registering heterogenuous
//! service objects representing services that can be
//! invoked. The service objects can be stateful.

pub mod services;

pub use services::*;

use std::any::Any;

use thiserror::Error;

/// Type of errors returned in attempting to access and use
/// a [ServiceMap].
#[derive(Error, Debug)]
pub enum ServiceMapError {
    #[error("incorrect service API type")]
    IncorrectAPIType,
    #[error("invalid service key")]
    InvalidKey,
}

/// Result type for `ServiceMapError`.
pub type ServiceResult<T> = Result<T, ServiceMapError>;

/// A "service map" is a heterogeneous collection of objects
/// representing services.
#[derive(Default)]
pub struct ServiceMap(Vec<Box<dyn Any + 'static>>);

impl ServiceMap {
    /// Register some boxed `package` object as a service in the
    /// map. Return the service key, which can be used to access
    /// this particular service object later.
    pub fn register(&mut self, package: Box<dyn Any + 'static>) -> usize {
        let key = self.0.len();
        self.0.push(package);
        key
    }

    /// Find the service at `key` in this map and return a
    /// mutable reference to it.
    pub(crate) fn get_service<T: 'static>(&mut self, key: usize) -> ServiceResult<&mut T> {
        let entry = self.0.get_mut(key).ok_or(ServiceMapError::InvalidKey)?;
        entry
            .downcast_mut::<T>()
            .ok_or(ServiceMapError::IncorrectAPIType)
    }
}
