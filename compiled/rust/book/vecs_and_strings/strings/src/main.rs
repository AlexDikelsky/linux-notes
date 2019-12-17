fn main() {
    let mut a = String::new();
    let b = String::from("ABC");
    a.push_str("ABCDEFGHIJ");

    let d = a + &b;

    let e = format!("{}-{}-{}", b, d, b);
   
    println!("{}", e);

    for ele in e.chars() {
        println!("{}", ele);
    }
}
