fn length(v: &Vec<i32>) -> u32 {
    //Have to create an empty list to compare it to
    let a: Vec<i32> = Vec::new();
    if v == &a {
        return 0;
    }
    else {
        length(&v[1..].into()) + 1
    }
}

fn main() {
    let mut v = Vec::new();
    for i in 0..10{
        println!("{}, {}", i, length(&v));
        v.push(i);
    }
}
