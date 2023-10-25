fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    println!("Hello, world!");
    let x = five();

    println!("The value of x is: {}", x);

    let x = plus_one(5);

    println!("The value of x is: {}", x);

    another_function();
    value(5);
    print_labeled_measurement(5, 'h');

    // this will be an error because unlike in other languages assigning a variable doesnt return
    // the value
    // let x = (let y = 6);

    // this works bc we return x+1
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}

fn another_function() {
    println!("another function");
}

fn value(x: i32) {
    println!("the value of x is {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

