#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect);

    println!("The rectangle is: {:#?}", rect);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect)
    );
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
