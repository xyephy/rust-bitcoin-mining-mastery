fn print_miner(name: &String) { // Borrow, don’t take
    println!("Borrowed miner: {}", name);
}

fn main() {
    let miner = String::from("Satoshi");
    print_miner(&miner); // Pass reference
    println!("Still got miner: {}", miner); // Works!
}
