use mozart::Cache;

fn main() {
    let mut cache = Cache::new(0, false);

    cache.set("Example", "123").unwrap();

    let result = cache.get("Example").unwrap();

    println!("{}", result)
}
