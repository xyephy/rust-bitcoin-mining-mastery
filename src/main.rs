struct BlockHeader {
    prev_hash: String,
    nonce: u32,
    difficulty: u32,
}

fn print_block(header: &BlockHeader) {
    println!(
        "Block - Prev Hash: {}, Nonce: {}, Difficulty: {}",
        header.prev_hash, header.nonce, header.difficulty
    );
}

fn fake_hash(nonce: u32) -> u32 {
    (nonce * 17) % 200 // Simpler, caps at 200—gives us a shot at < 100
}

fn mine_block(header: &mut BlockHeader) -> bool {
    for n in 0..100 { // Still 100 tries
        header.nonce = n;
        let hash = fake_hash(n);
        println!("Trying nonce: {}, Hash: {}", n, hash); // Debug line—see the hunt!
        if hash < header.difficulty {
            println!("Mined! Hash: {}, Nonce: {}", hash, n);
            return true;
        }
    }
    false
}

fn main() {
    let mut block = BlockHeader {
        prev_hash: String::from("0000abc..."),
        nonce: 0,
        difficulty: 100,
    };

    print_block(&block);
    if mine_block(&mut block) {
        println!("Block mined successfully!");
    } else {
        println!("Mining failed—nonce limit hit.");
    }
    print_block(&block);
}
