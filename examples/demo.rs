use servicemap::*;

fn main() {
    let mut map = ServiceMap::default();

    let counter_key = map.register::<Counter>();
    let joiner_key = map.register::<Joiner>();

    let _ = map.invoke::<Counter>(counter_key, 7).unwrap();
    let count = map.invoke::<Counter>(counter_key, 8).unwrap();
    println!("{count}");

    let hello = "hello".to_string();
    let join = map.invoke::<Joiner>(joiner_key, (&hello, "world")).unwrap();
    println!("{join}");
}
