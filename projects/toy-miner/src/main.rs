use sha2::{Digest, Sha256};

struct BlockHeader {
    version: u32,
    prev_hash: String,
    merkle_root: String,
    timestamp: u32,
    bits: u32, // Difficulty in compact form
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
    Sha256::digest(&first_hash).to_vec()
}

// Simplified difficulty check (bits -> target)
fn check_difficulty(hash: &[u8], bits: u32) -> bool {
    // For bits 0x1d00ffff (easy), target is ~0x00000000FFFF0000...
    let target = vec![0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00]; // 8 bytes for simplicity
    for i in 0..8 {
        if hash[i] > target[i] {
            return false;
        } else if hash[i] < target[i] {
            return true;
        }
    }
    true // Equal is OK
}

fn mine(header: &mut BlockHeader) -> bool {
    for n in 0..100000 {
        header.nonce = n;
        let hash = double_sha256(header);
        if check_difficulty(&hash, header.bits) {
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
        timestamp: 1677654321,
        bits: 0x1d00ffff, // Easy Bitcoin difficulty
        nonce: 0,
    };
    println!("Mining block with bits: {:x}", block.bits);
    if mine(&mut block) {
        println!("Success!");
    } else {
        println!("Failed after 100k attempts.");
    }
}
