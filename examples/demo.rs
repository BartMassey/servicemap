use servicemap::*;

fn main() {
    let mut map = ServiceMap::default();

    let counter_key = map.register(Counter::package());
    let joiner_key = map.register(Joiner::package());

    let _ = Counter::invoke(&mut map, counter_key, 7).unwrap();
    let count = Counter::invoke(&mut map, counter_key, 8).unwrap();
    println!("{count}");

    let hello = "hello".to_string();
    let join = Joiner::invoke(&mut map, joiner_key, &hello, "world").unwrap();
    println!("{join}");
}
