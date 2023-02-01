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
        let counter = entry
            .downcast_mut::<Counter>()
            .ok_or(ServiceMapError::IncorrectAPIType)?;
        Ok(counter.increase(n))
    }
}

impl ServicePackage for Counter {
    fn package() -> Box<dyn Any + 'static> {
        <Box<Counter>>::default()
    }
}
