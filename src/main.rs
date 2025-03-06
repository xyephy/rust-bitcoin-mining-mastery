struct BlockHeader {
    prev_hash: String,
    nonce: u32,
    difficulty: u32,
}

fn print_block(header: &BlockHeader) {
    println!("Block nonce: {}", header.nonce);
}

fn update_nonce(header: &mut BlockHeader, new_nonce: u32) {
    header.nonce = new_nonce;
}

fn main() {
    let mut block = BlockHeader {
        prev_hash: String::from("0000abc..."),
        nonce: 0,
        difficulty: 1000,
    };

    print_block(&block); // Immutable borrow
    update_nonce(&mut block, 42); // Mutable borrow
    print_block(&block); // Back to immutable
    println!("Updated nonce: {}", block.nonce);

    // Multiple immutable borrows
    let ref1 = &block;
    let ref2 = &block;
    println!("Double borrow: {} and {}", ref1.nonce, ref2.nonce);
}
