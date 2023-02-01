use crate::*;

/// The counter service provides an internal monotonically
/// increasing counter.
#[derive(Default)]
pub struct Counter(usize);

impl Counter {
    /// Perform this service by increasing the counter
    /// value by `n`.
    fn increase(&mut self, n: usize) -> usize {
        self.0 += n;
        self.0
    }

    /// Package this service with default initial
    /// conditions (counter at 0).
    pub fn package() -> Box<Counter> {
        <Box<Counter>>::default()
    }

    /// Run the counter service and return the result.
    pub fn invoke(map: &mut ServiceMap, key: ServiceKey, n: usize) -> ServiceResult<usize> {
        let counter = map.get_service::<Counter>(key)?;
        Ok(counter.increase(n))
    }
}
