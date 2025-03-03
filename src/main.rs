fn main() {
    let miner = "Satoshi"; // Immutable
    println!("Miner: {}", miner);

    let mut hash_rate = 50; // Mutable
    println!("Hash rate: {} MH/s", hash_rate);
    hash_rate = 75; // Reassign
    println!("Updated hash rate: {} MH/s", hash_rate);

    let hash_rate = hash_rate + 25; // Shadowing
    println!("Boosted hash rate: {} MH/s", hash_rate);
}
