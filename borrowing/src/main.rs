fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s1 = String::from("hello");

    // references are immutable by default as well
    // if you have a mutable reference it can be the only reference to a value
    let changed = change(&mut s1);

    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    // this isn't ok
    // let r2 = &mut s;
    //
    // let mut s = String::from("hello");
    //
    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM
    //
    // println!("{}, {}, and {}", r1, r2, r3);

    // but this is
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// this isn't allowed by the compiler as it would create a dangling reference
// fn dangle() -> &String {
//     let s = String::from("hello");
//
//     &s
// }

// the solution is to just return s
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
