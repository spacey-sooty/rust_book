fn main() {
    std::env::set_var("RUST_BACKTRACE", "full");
    let v = vec![1, 2, 3];

     v[99];
}
