use rand::Rng;

fn main() {
    let num = rand::thread_rng().gen_range(0..100);
    println!("Hello, world! {num} plus one is {}!", add_one::add_one(num));
}
