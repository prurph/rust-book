use std::ops::Deref;

fn main() {
    let x = 5;
    let y = MyBox::new(5);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    // Deref coercion: &MyBox<String> -> &String -> &str
    // MyBox<T> implements Deref<Target=T>
    // String implements Deref<Target=str>
    hello(&m);
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
