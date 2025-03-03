fn check_difficulty(hash: i32, difficulty: i32) -> bool {
    hash < difficulty // Simulating "did we find a valid hash?"
}

fn main() {
    let target_difficulty = 1000;
    let mut nonce = 0;

    loop {
        nonce += 1;
        let hash = nonce * 42; // Fake hash calc
        println!("Trying nonce: {}, hash: {}", nonce, hash);

        if check_difficulty(hash, target_difficulty) {
            println!("Found valid hash {} with nonce {}!", hash, nonce);
            break;
        }

        if nonce > 10 { // Cap for now 
            println!("No luck yet...");
            break;
        }
    }
}
