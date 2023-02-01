use crate::*;

#[derive(Default)]
pub struct Counter(usize);

impl Counter {
    fn increase(&mut self, n: usize) -> usize {
        self.0 += n;
        self.0
    }

    pub fn package() -> Box<Counter> {
        <Box<Counter>>::default()
    }

    pub fn invoke(map: &mut ServiceMap, key: usize, n: usize) -> ServiceResult<usize> {
        let counter = map.get_service::<Counter>(key)?;
        Ok(counter.increase(n))
    }
}
