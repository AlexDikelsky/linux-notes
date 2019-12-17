//Prints a triagle with base=n
//TODO: Make it work for n%2=1 triangles
fn make_triangle(base: i32) -> () {
    for x in 0..base {
        for y in 0..base {
            if x/2+base/2 == y {
                print!("X");
            }
            else if base/2-1-x/2 == y {
                print!("Z");
            }
            else if base-1 == x{
                print!("Y");
            }
            else{
                print!("-");
            }
        }
        println!();
    }
}

fn main() {
    make_triangle(16)
}
