fn main() {
    let s = String::from("hello world");

    let h = &s[..5];
    let w = &s[6..];
    let whole = &s[..];
    let slice_of_slice = &h[2..3];
    
    println!("{}, aa {}", h, w);
    println!("{}", whole);
    println!("{}", slice_of_slice);

    let a = [7, 3, 6, 1, 5];
    let b = &a[0..3];
    println!("List of random vals");
    for ele in a.iter() {
        println!("{}", ele);
    }
    println!("Sliced with 0..3");
    for ele in b.iter() {
        println!("{}", ele);
    }
    println!("len of slice from 0 to 3 = {}", b.len());
}
