fn isGreater(a: u32, b: u32) -> bool {
    if b == a {
        return false
    }
    else if b == 0 {
        return true 
    }
    else if a == 0 {
        return false
    }
    else {
        return isGreater(a-1, b-1)
    }
}

fn main() {
    for i in 0..5 {
        for j in 0..5{
            println!("{} > {} is {}", i, j, isGreater(i, j));
        }
    }
}
