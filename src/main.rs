struct BlockHeader {
    prev_hash: String, // Simplified hash
    nonce: u32,        // Mining variable
    difficulty: u32,   // Target threshold
}

fn check_block(header: &BlockHeader) -> bool {
    let fake_hash = header.nonce * 10; // Dummy calc
    fake_hash < header.difficulty
}

fn main() {
    let block = BlockHeader {
        prev_hash: String::from("0000abc..."), // Fake hash
        nonce: 42,
        difficulty: 1000,
    };

    if check_block(&block) {
        println!("Block valid with nonce {}!", block.nonce);
    } else {
        println!("Nonce {} too weak.", block.nonce);
    }

    // Ownership still intact
    println!("Prev hash: {}", block.prev_hash);
}
