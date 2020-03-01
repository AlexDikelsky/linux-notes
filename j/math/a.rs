fn main() {
    dbg!(s(10000000));
}

fn b(x: u64) -> u64 {
    match x == 0 {
        true => 0,
        false => (x / 2) + b(x/2),
    }
}

fn g(x: u64, y: u64) -> u64 {
    b(x)-b(y)-b(x-y)
}

fn max_thing(x: u64) -> u64 {
    (0..(x+1)).map(|y| g(x, y)).max().unwrap()
}

fn s(x: u64) -> u64 {
    (0..(x+1)).map(|y| max_thing(y)).sum()
}
