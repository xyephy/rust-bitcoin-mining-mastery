fn main() {
    let miner = String::from("Satoshi"); // Owned string
    println!("Miner: {}", miner);

   let miner2 = miner; // Move (not copy!)
   // println!("Miner: {}", miner); // This errors—miner’s gone!
    println!("Miner2: {}", miner2);

    let hash_rate = 42; // Primitive, copies fine
    let hash_rate2 = hash_rate; // No move here
    println!("Hash rates: {} and {}", hash_rate, hash_rate2);
}
