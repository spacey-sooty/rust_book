// consts can be declared at any scope
// they are evaluated at compile time
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn mut_example() {
    // without mut this breaks due to variables being immutable by default
    // let x = 5;
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

fn scope_and_shadowing() {
    let x = 5;

    // we can do this because we are redeclaring the variable
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    // this value is from line 17 due to change of scope
    println!("The value of x is: {x}");

    // when we do this it redeclares the variable
    // let mut spaces then removing the let keyword from line 31 as spaces.len and "  " are of
    // different types
    let spaces = "  ";
    let spaces = spaces.len();
}

fn main() {
    mut_example();
    scope_and_shadowing();
}

