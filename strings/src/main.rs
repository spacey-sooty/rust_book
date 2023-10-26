fn main() {
    let mut s = String::new();

    let data = "initial contents";

    s = data.to_string();

    // or alternatively

    let s = String::from("initial contents");

    // strings are utf 8 encoded so nearly anything is valid
    // eg :
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // changing strings

    let mut s = String::from("foo");
    // takes str slice
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    let mut s = String::from("lo");
    // takes single character
    s.push('l');

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
                       // because + is implemented similar to this
                       // fn add(self, s: &str) -> String {

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // this would be complex using the + operator
    let s = format!("{}-{}-{}", s1, s2, s3);

    // you cant index strings to avoid bugs caused by unnexpected values
    // you can slice but do so with caution

    for c in "Зд".chars() {
        println!("{}", c);
    }
    // returns Зд

    for b in "Зд".bytes() {
        println!("{}", b);
    }
    // returns 208 151 208 180
}
