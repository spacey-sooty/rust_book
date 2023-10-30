fn main() {
    std::env::set_var("RUST_BACKTRACE", "full"); // normally 1 is fine here
    let v = vec![1, 2, 3];

    v[99];
}
