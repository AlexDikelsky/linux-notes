fn main () {
    let ex = |x| x;

    //One of these can work, but not both
    //This is because it gets defined by the first call
 
    //let s = ex(String::from("hello")); //Here it's defined as a string returning a string
    
    let s = ex(5); //Here it's defied as a i32 returning an i32
    let r = ex(8);

    println!("{}, gdf", s);

    //This is legal, but declaring it without calling it or giving types throws an error
    let ex_2 = |x: u32| -> u32 { x + 1 }; 
}
