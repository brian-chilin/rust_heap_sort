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

    print!("New tree: ");
    display_tree(&array);
    //print!("{:?}", array);

    print!("Enter to continue");
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    //print!("{}", input);

    heapsort(&mut array);

    print!("Finished sorting: ");
    display_tree(&array);
    print!("{:?}", array);
}

fn display_tree(array: &[u8; 31]) {
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
    array[0], array[1], array[2], array[3], array[4], array[5], array[6], array[7], array[8], array[9], array[10], array[11], array[12], array[13], array[14], array[15],
             array[16], array[17], array[18], array[19], array[20], array[21], array[22], array[23], array[24], array[25], array[26], array[27], array[28], array[29], array[30]);
}

fn heapsort(array: &mut [u8; 31]) {
    build_max_heap(array);
    print!("\nAs a max-heap: ");
    display_tree(array);

    for i in 0..30 {
        let temp = array[0];
        array[0] = array[30-i];
        array[30-i] = temp;
        println!("{:?}", array); // instruction 5a
        max_heapify(array, 0, 30-i);
    }

    //_sorted(array);
}

fn _sorted(array: &[u8]) {
    let mut last = array[0];
    for i in 1..array.len() {
        if array[i] < last {
            print!("unsorted, {} shouldn't come after {}", array[i], last);
            return
        }
        last = array[i];
    }
    print!("sorted!");
}

fn build_max_heap(array: &mut [u8]) {
    //this among other heap functions i copy from
    //Introduction to Algorithms 3ed by Thomas H. Cormen
    //i studied and learned these well last semester in CECS328
    for i in (0..(array.len()/2)).rev() {
        max_heapify(array, i, array.len());
    }
}

fn max_heapify(array: &mut [u8], index: usize, heap_size: usize) {
    // heap_size =< array.len() should be true
    let left_i  = (index * 2) + 1;
    let right_i = (index * 2) + 2;

    let mut largest = index;
    if left_i < heap_size && array[left_i] > array[largest]{
        largest = left_i;
    }
    if right_i < heap_size && array[right_i] > array[largest] {
        largest = right_i
    }
    if largest != index {
        let temp = array[index];
        array[index] = array[largest];
        array[largest] = temp;
        max_heapify(array, largest, heap_size);
    }
}