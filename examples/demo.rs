use servicemap::*;

fn main() {
    let mut map = ServiceMap::default();

    let counter = Counter::default().package();
    let counter_key = map.register(counter);
    let joiner = Joiner::default().package();
    let joiner_key = map.register(joiner);

    let _ = map.invoke(counter_key, &7usize);
    let count = map.invoke(counter_key, &8usize);
    println!("{}", Counter::result(count));

    let join = map.invoke(joiner_key, &("hello", "world"));
    println!("{}", Joiner::result(join));
}
