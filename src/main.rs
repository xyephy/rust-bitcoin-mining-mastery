struct BlockHeader {
    prev_hash: String,
    nonce: u32,
    difficulty: u32,
}

fn print_block<'a>(header: &'a BlockHeader) -> &'a str {
    println!("Block - Prev: {}, Nonce: {}, Diff: {}", header.prev_hash, header.nonce, header.difficulty);
    &header.prev_hash
}

fn fake_hash(nonce: u32) -> u32 {
    (nonce * 17) % 200 // From Day 4
}

fn main() {
    let mut block = BlockHeader {
        prev_hash: String::from("0000abc..."),
        nonce: 0,
        difficulty: 100,
    };
    let prev = print_block(&block);
    println!("Prev hash returned: {}", prev);
    block.nonce = fake_hash(42); // Use it
    print_block(&block);
}
