use sha2::{Digest, Sha256};

struct BlockHeader {
    version: u32,
    prev_hash: String,
    merkle_root: String,
    timestamp: u32,
    bits: u32, // Difficulty target (simplified)
    nonce: u32,
}

fn double_sha256(header: &BlockHeader) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(header.version.to_le_bytes());
    hasher.update(&header.prev_hash);
    hasher.update(&header.merkle_root);
    hasher.update(header.timestamp.to_le_bytes());
    hasher.update(header.bits.to_le_bytes());
    hasher.update(header.nonce.to_le_bytes());
    let first_hash = hasher.finalize();
    Sha256::digest(&first_hash).to_vec() // Double hash
}

fn mine(header: &mut BlockHeader) -> bool {
    for n in 0..100000 {
        // Bigger range—mining’s hard!
        header.nonce = n;
        let hash = double_sha256(header);
        if hash[0] == 0 && hash[1] == 0 {
            // 2 leading zeros
            let hash_hex = hash
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect::<String>();
            println!("Mined! Nonce: {}, Hash: {}", n, hash_hex);
            return true;
        }
    }
    false
}

fn main() {
    let mut block = BlockHeader {
        version: 1,
        prev_hash: String::from("0000abc..."),
        merkle_root: String::from("deadbeef..."),
        timestamp: 1677654321, // Fake timestamp
        bits: 0x1d00ffff,      // Easy difficulty
        nonce: 0,
    };

    println!("Mining block...");
    if mine(&mut block) {
        println!("Success!");
    } else {
        println!("Failed after 100k attempts.");
    }
}
