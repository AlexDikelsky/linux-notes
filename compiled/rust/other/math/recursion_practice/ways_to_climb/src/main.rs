//Takes int, returns list of possible combos to get to finsh line
fn ways_to_climb(n: i32) -> i32 {
    if n < 0 {
        0
    }
    else if n == 0{
        1
    }
    else {
        ways_to_climb(n-1) + ways_to_climb(n-2) + ways_to_climb(n-3)
    }
}


fn main() {
    for n in 0..20 {
        println!("{}: {}", n, ways_to_climb(n));
    }
}
