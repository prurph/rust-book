#[allow(dead_code)]
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

impl Coin {
    fn value_in_cents(&self) -> u8 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}

enum Location {
    Point(i32),
    Range(i32, i32),
}

// A useless function to awkwardly demonstrate if let
fn print_range_max(loc: &Location) {
    if let Location::Range(x, y) = loc {
        if y > x {
            println!("{}", y);
        } else {
            println!("{}", x);
        }
    }
}

fn main() {
    let coin = Coin::Quarter;

    println!("A {:?} is worth {} cents", coin, coin.value_in_cents());

    let some1 = Some(1);
    let none: Option<i32> = None;

    println!("{:?} + 1 = {:?}", some1, plus_one(some1));
    println!("{:?} + 1 = {:?}", none, plus_one(none));

    // Owned string
    let opt = Some(String::from("Hello world"));

    match opt {
        Some(_) => println!("Some"),
        None => println!("None"),
    }

    // This is fine, the value wasn't captured in the match, so it was never moved
    println!("{:?}", opt);

    // Must match on &opt if we want to capture the value because `String` doesn't implement
    // `Copy` and the capture of owned value moves the value. Rust "pushes down" the enum reference
    // &Option<String> to &String, so s has type &String, and opt can be used after the match.
    match &opt {
        Some(s) => println!("Some: {}", s),
        None => println!("None"),
    }

    println!("{:?}", opt);

    let i32_opt = Some(1);

    // This, however, is fine: i32 implements copy, so capturing it can move it into i
    // without removing i32_opt's ownership, allowing us to print it later.
    match i32_opt {
        Some(i) => println!("{}", i),
        None => println!("None"),
    }

    println!("{:?}", i32_opt);

    print_range_max(&Location::Point(1));
    print_range_max(&Location::Range(1, 2));
}
