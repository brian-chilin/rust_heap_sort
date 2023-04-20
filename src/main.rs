use rand::Rng;
use std::collections::HashSet;

// https://doc.rust-lang.org/rust-by-example/std/hash/hashset.html

fn main() {
    let mut rng = rand::thread_rng();

    let mut heap: [u8; 32] = [0; 32];
    let mut used_nums = HashSet::new();

    for i in 0..32 {
        let n = rng.gen_range(10..=99);

        // while loop here to keep generating n if it already exists
        used_nums.insert(n);
        print!("{} ", (used_nums.contains(&n)));

        heap[i] = n;
        print!("{} ", heap[i]);
    }
}
