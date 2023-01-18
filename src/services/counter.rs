use crate::*;

#[derive(Default)]
pub struct Counter(usize);

impl Counter {
    fn increase(&mut self, n: usize) -> usize {
        self.0 += n;
        self.0
    }

    pub fn package(self) -> ServiceEntry {
        ServiceEntry(Box::new(self))
    }

    pub fn result(r: Box<dyn Any>) -> usize {
        *r.downcast::<usize>().unwrap()
    }
}

impl Service for Counter {
    fn call(&mut self, args: &dyn Any) -> Box<dyn Any> {
        let args: &usize = args.downcast_ref::<usize>().unwrap();
        Box::new(self.increase(*args))
    }
}
