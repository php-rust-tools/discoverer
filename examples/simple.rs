use discoverer::discover;

fn main() {
    let discoveries = discover("txt", &[
        &concat!(env!("CARGO_MANIFEST_DIR"), "/examples/fixtures")
    ]).unwrap();

    dbg!(discoveries);
}