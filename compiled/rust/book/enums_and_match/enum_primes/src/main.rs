#[derive(PartialEq)]
#[derive(Debug)]
enum NumKind {
    Prime,
    Composite,
}

struct Number {
    kind: NumKind,
    value: u32,
}

fn choose_number(n: u32) -> Number {
    Number { value: n, kind: is_prime(n) }
}

fn is_prime(n: u32) -> NumKind {
    let mut i = 2;
    let mut num_type = NumKind::Prime;
    
    while num_type == NumKind::Prime && i < n {
        if n % i == 0 {
            num_type = NumKind::Composite;
        }
        i += 1;
    }
    num_type
}

fn main() {
    let a: Number = choose_number(8);
    let b: Number = choose_number(11);
    
    println!("{}", a.value);
    println!("{:#?}", a.kind);
    
    println!("{}", b.value);
    println!("{:#?}", b.kind);
}
