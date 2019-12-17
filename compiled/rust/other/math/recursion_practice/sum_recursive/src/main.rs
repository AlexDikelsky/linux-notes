//Subtracts one from b until it equals 0
//While doing that, add 1 to a
fn sum(a: u32, b: u32) -> u32 {
    if b == 0 {
        return a
    }
    else {
        return sum(a+1, b-1)
    }
}

fn main() {
    for i in 0..5 {
        for j in 0..5 {
            println!("{} + {} = {}", i, j, sum(i, j));
        }
    }
}
