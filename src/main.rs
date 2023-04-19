use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    let a: u32 = rng.gen();
    let b: u32 = rng.gen();

    println!("a: {}\nb: {}", a, b);
}
