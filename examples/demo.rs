use servicemap::*;

fn main() {
    let mut map = ServiceMap::default();
    let counter = Counter::default().package();
    let key = map.register(counter);
    let _ = map.invoke(key, &7usize);
    let count = map.invoke(key, &8usize);
    println!("{}", Counter::result(count));
}
