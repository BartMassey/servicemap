use crate::*;

#[derive(Default)]
pub struct Counter(usize);

impl Counter {
    fn increase(&mut self, n: usize) -> usize {
        self.0 += n;
        self.0
    }
}

impl ServicePackage for Counter {
    type CallArgs = usize;
    type CallResult = usize;

    fn package() -> ServiceEntry {
        ServiceEntry(Box::new(Counter::default()))
    }
}

impl Service for Counter {
    fn call(&mut self, args: &dyn Any) -> ServiceResult<Box<dyn Any>> {
        let args: &usize = args.downcast_ref::<usize>()
            .ok_or(ServiceMapError::IncorrectArgumentType)?;
        Ok(Box::new(self.increase(*args)))
    }
}
