use sha2::{Digest, Sha256};

struct BlockHeader {
    prev_hash: String,
    nonce: u32,
    difficulty: u32,
    data: String, // New field
}

fn print_block(header: &BlockHeader) {
    println!("Block - Prev: {}, Nonce: {}, Diff: {}", header.prev_hash, header.nonce, header.difficulty);
}

fn hash_block(header: &BlockHeader) -> String {
    let mut hasher = Sha256::new();
    hasher.update(&header.prev_hash);
    hasher.update(header.nonce.to_string());
    hasher.update(&header.data);
    format!("{:x}", hasher.finalize())
}

fn mine_block(header: &mut BlockHeader) -> bool {
    for n in 0..10000 {
        header.nonce = n;
        let hash = hash_block(header);
        if hash.starts_with("00") { // Difficulty: 2 zeros
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
        difficulty: 2,
        data: String::from("tx: 1 BTC"),
    };
    print_block(&block);
    if mine_block(&mut block) {
        println!("Block mined successfully!");
    } else {
        println!("Mining failed—nonce limit hit.");
    }
    print_block(&block);
}
