#![allow(unused)]

use std::any::Any;

trait Service<'a> {
    fn package(self) -> Box<dyn Service<'a> + 'a>;
    fn call(&mut self, args: &dyn Any) -> Box<dyn Any>;
}

#[derive(Default)]
struct Counter(usize);

impl Counter {
    fn increase(&mut self, n: usize) -> usize {
        self.0 += n;
        self.0
    }

    fn result(r: Box<dyn Any>) -> usize {
        *r.downcast::<usize>().unwrap()
    }
}

impl<'a> Service<'a> for Counter {
    fn package(self) -> Box<dyn Service<'a> + 'a> {
        Box::new(self)
    }

    fn call(&mut self, args: &dyn Any) -> Box<dyn Any> {
        let args: &usize = args.downcast_ref::<usize>().unwrap();
        Box::new(self.increase(*args))
    }
}

#[derive(Default)]
struct ServiceMap<'a>(Vec<Box<dyn Service<'a> + 'a>>);

impl<'a> ServiceMap<'a> {
    fn register(&mut self, service: Box<dyn Service<'a> + 'a>) -> usize {
        let key = self.0.len();
        self.0.push(service);
        key
    }

    fn invoke(&mut self, key: usize, args: &dyn Any) -> Box<dyn Any> {
        self.0[key].call(args)
    }
}

fn main() {
    let mut map = ServiceMap::default();
    let counter = Counter::default().package();
    let key = map.register(counter);
    let count = map.invoke(key, &7usize);
    let count = map.invoke(key, &8usize);
    println!("{}", Counter::result(count));
}
