#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(15 * scale),
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    dbg!(&rect1);

    println!("rect1 is: {:#?}", rect1);

    println!("The area of the rect1 is {} square pixels.", &rect1.area());

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let mut square1 = Rectangle::square(10);

    println!("The area of square1 is {} square pixels.", &square1.area());

    square1.set_width(100);

    println!(
        "Now the area of square1 is {} square pixels (and it's no longer a square).",
        &square1.area()
    );
}
