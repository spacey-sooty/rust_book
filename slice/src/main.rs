fn main() {
    let mut s = String::from("hello world");

    // two string slice examples
    let hello = &s[0..5];
    let world = &s[6..11];

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!

    // a &str points to a specific spot in the binary
    let s = "Hello World";
}

// this becomes hard to track as usize is only something with the string it comes from
// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();
//
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//
//     s.len()
// }

// using slices we can avoid having to track the existence of the parent string
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// there is also a more general slice type

