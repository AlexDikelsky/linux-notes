fn q(x: isize, y: isize) -> isize {
    if x < y {
        0
    } else {
        println!("{}", x);
        1+q(x-y, y)
//        println!("Never EXECUTES");
//        c
    }
}
fn main() {
    println!("{}", q(9807671932, 2));
}
