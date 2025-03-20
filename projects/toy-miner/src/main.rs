use sha2::{Digest, Sha256};

struct BlockHeader {
    version: u32,
    prev_hash: String,
    merkle_root: String,
    timestamp: u32,
    bits: u32,
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

fn check_difficulty(hash: &[u8], bits: u32) -> bool {
    let target = vec![0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00]; // Easy target
    for i in 0..8 {
        if hash[i] > target[i] { return false; }
        if hash[i] < target[i] { return true; }
    }
    true
}

fn mine(header: &mut BlockHeader) -> bool {
    let mut attempts = 0;
    for n in 0..1000000 { // Up range—real mining’s grindy
        header.nonce = n;
        attempts += 1;
        let hash = double_sha256(header);
        if check_difficulty(&hash, header.bits) {
            let hash_hex = hash.iter().map(|b| format!("{:02x}", b)).collect::<String>();
            println!("Mined! Nonce: {}, Hash: {}, Attempts: {}", n, hash_hex, attempts);
            return true;
        }
    }
    println!("Failed after {} attempts", attempts);
    false
}

fn main() {
    let mut block = BlockHeader {
        version: 1,
        prev_hash: String::from("0000abc..."),
        merkle_root: String::from("deadbeef..."),
        timestamp: 1677654321,
        bits: 0x1d00ffff,
        nonce: 0,
    };
    println!("Mining block with bits: {:x}", block.bits);
    if mine(&mut block) {
        println!("Success! Nonce: {}", block.nonce);
    } else {
        println!("Failed!");
    }
}
