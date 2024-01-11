use std::mem::drop;

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let _c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointer instances created.");

    let e = CustomSmartPointer {
        data: String::from("std::mem::drop drop"),
    };
    println!("CustomSmartPointer for manual drop created");
    drop(e);
    println!("CustomSmartPointer for manual drop dropped");
}
