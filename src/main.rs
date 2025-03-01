fn greet_miner(name: &str, hash_rate: i32) {
    println!("Hello, {}! Hash rate: {} MH/s", name, hash_rate);
}
fn main() {
    greet_miner("Bitcoin Miner", 42);
}
