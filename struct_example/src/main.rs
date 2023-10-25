// we can also give more functionality using traits
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    // we could also do this using tuple
    let rect = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        tuple_area(rect)
    );

    // we could also refactor this with structs to avoid having to explain the fields etc

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        struct_area(&rect1)
    );

    // this would not work originally because it doesn't implement std::fmt::Display
    // println!("The rectangle is {}", rect1);

    // we can now use this because of the derived debug trait
    // :#? is often more readable but both work
    println!("The rectangle is {:?}", rect1);

    // we can also print it to stderr
    dbg!(rect1);
    // this macro returns ownership of the value after printing to stderr
    // this means we could assign a struct field using something like dbg!(60)
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn tuple_area(rect: (u32, u32)) -> u32 {
    rect.0 * rect.1
}

fn struct_area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
