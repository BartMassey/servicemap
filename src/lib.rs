pub mod services;

pub use services::*;

use std::any::Any;

pub struct ServiceEntry(Box<dyn Service>);

pub(crate) trait Service {
    fn call(&mut self, args: &dyn Any) -> Box<dyn Any>;
}

#[derive(Default)]
pub struct ServiceMap(Vec<ServiceEntry>);

impl ServiceMap {
    pub fn register(&mut self, service: ServiceEntry) -> usize {
        let key = self.0.len();
        self.0.push(service);
        key
    }

    pub fn invoke(&mut self, key: usize, args: &dyn Any) -> Box<dyn Any> {
        self.0[key].0.call(args)
    }
}
