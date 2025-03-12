# Week 1: March 1-7 - Rust & Bitcoin Basics
## March 1
- Setup: Rust env ready, GitHub repo live.
- Code: First Rust program done.
- Played with i32 for hash rate fun.

## March 3
- Studied: Variables (mutability, shadowing) and control flow (if, loop) via [Rust by Example](https://doc.rust-lang.org/rust-by-example/).
- Code: Simulated a basic mining loop with nonce and fake hash ([src/main.rs](../src/main.rs)).
- Notes: Rust’s strictness on mutability is wild but makes sense for safety.

## March 4
- Studied: Ownership (moves, borrowing) via [Rust by Example](https://doc.rust-lang.org/rust-by-example/).
- Code: Built a Bitcoin block header struct ([src/main.rs](../src/main.rs)).
- Notes: Ownership errors are brutal but make sense—& saved me!

## March 5 
- Studied: Borrowing and lifetimes via [Rust by Example](https://doc.rust-lang.org/rust-by-example/).
- Code: Mining sim with BlockHeader ([src/main.rs](../src/main.rs)).
- Notes: Mining failed at first tweaked fake_hash, now it mines!

## March 12 (Days 5-7)
- Studied: 
  - Day 5: Lifetimes, structs via [Rust by Example](https://doc.rust-lang.org/rust-by-example/).
  - Day 6: Bitcoin intro, SHA-256 hashing.
  - Day 7: Recap, miner polish with attempt counter.
- Code: SHA-256 miner with attempts ([src/main.rs](../src/main.rs)).
- Notes: Week 1 crammed into today—Rust’s ownership and Bitcoin’s mining logic are clicking!

