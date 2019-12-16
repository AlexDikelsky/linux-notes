fn prime_no_mod(n: usize) -> bool {
    let mut x = 1;
    while x < n {
        let mut y = 1;
        while y < n {
            if x*y == n {
                true
            }
            y += 1;
        }
        x += 1;
    }
    false
}
fn main() {
    println!("t, {}", prime_no_mod(8));
    println!("t, {}", prime_no_mod(15));
    println!("t, {}", prime_no_mod(100));
    println!("f, {}", prime_no_mod(5));
    println!("f, {}", prime_no_mod(23));
}
