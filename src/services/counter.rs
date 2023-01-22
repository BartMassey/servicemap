use crate::*;

#[derive(Default)]
pub struct Counter(usize);

impl Counter {
    fn increase(&mut self, n: usize) -> usize {
        self.0 += n;
        self.0
    }

    pub fn invoke(map: &mut ServiceMap, key: usize, n: usize) -> ServiceResult<usize> {
        let entry = map.0.get_mut(key).ok_or(ServiceMapError::InvalidKey)?;
        let result = entry.call(&n)?;
        let result = *result.downcast_ref::<usize>()
            .ok_or(ServiceMapError::IncorrectResultType)?;
        Ok(result)
    }
}

impl ServicePackage for Counter {
    fn package() -> Box<dyn Service> {
        Box::new(Counter::default())
    }
}

impl Service for Counter {
    fn call(&mut self, args: &(dyn Any + 'static)) -> ServiceResult<Box<dyn Any + 'static>> {
        let args: usize = *args.downcast_ref::<usize>()
            .ok_or(ServiceMapError::IncorrectArgumentType)?;
        Ok(Box::new(self.increase(args)))
    }
}
