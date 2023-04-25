use rand::Rng;
use std::collections::HashSet;
use std::io::{Write};

// https://doc.rust-lang.org/rust-by-example/std/hash/hashset.html

fn main() {
    let mut rng = rand::thread_rng();

    let mut array: [u8; 31] = [0; 31];
    let mut used_nums = HashSet::new();

    for i in 0..31 {
        let mut n = rng.gen_range(10..=99);
        // while loop here to keep generating n if it already exists
        while used_nums.contains(&n) {
            n = rng.gen_range(10..=99);
        }
        used_nums.insert(n);
        array[i] = n;
    }

    display_tree(&array);
    //print!("{:?}", array);

    print!("Enter to continue");
    std::io::stdout().flush().unwrap();
    //let mut input = String::new();
    //std::io::stdin().read_line(&mut input).unwrap();
    //print!("{}", input);

    heapsort(&mut array);
}

fn display_tree(heap: &[u8; 31]) {
    println!("
                                            {}
   	                                         |
		     	     {}----------------------^----------------------{}
                     |                                               |
	     {}----------^----------{}                       {}----------^----------{}
          |                      |                       |                       |
    {}----^----{}		   {}----^----{}           {}----^----{}           {}----^----{}
    |           |          |           |           |           |           |           |
{}--^--{}   {}--^--{}  {}--^--{}   {}--^--{}   {}--^--{}   {}--^--{}   {}--^--{}   {}--^--{}",
    heap[0], heap[1], heap[2], heap[3], heap[4], heap[5], heap[6], heap[7], heap[8], heap[9], heap[10], heap[11], heap[12], heap[13], heap[14], heap[15],
             heap[16], heap[17], heap[18], heap[19], heap[20], heap[21], heap[22], heap[23], heap[24], heap[25], heap[26], heap[27], heap[28], heap[29], heap[30]);
}

fn heapsort(array: &mut [u8; 31]) {
    build_min_heap(array);
    display_tree(array);

    for i in 0..31 {
        println!("{}", i);
    }
}

fn build_min_heap(array: &mut [u8]) {
    //this among other heap functions i copy from
    //Introduction to Algorithms 3ed by Thomas H. Cormen
    //i studied and learned these well last semester in CECS328
    for i in (0..(array.len()/2)).rev() {
        min_heapify(array, i);
    }
}

fn min_heapify(array: &mut [u8], index: usize) {
    let left_i  = (index * 2) + 2;
    let right_i = (index * 2) + 1;

    let mut smallest = index;
    if left_i < array.len() && array[left_i] < array[smallest] {
        smallest = left_i;
    }
    if right_i < array.len() && array[right_i] < array[smallest] {
        smallest = right_i
    }
    if smallest != index {
        let temp = array[index];
        array[index] = array[smallest];
        array[smallest] = temp;
        min_heapify(array, smallest)
    }
}