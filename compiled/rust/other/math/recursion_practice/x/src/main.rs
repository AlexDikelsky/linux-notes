fn fact(n: usize) -> usize {
    match n {
        0 => 1,
        n => fact(n-1) * n
    }
}

fn main() {
    println!("{}", fact(5));
}

