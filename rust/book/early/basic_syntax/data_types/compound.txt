fn main() {
    //Makes a tuple, then deconstructs it.
    let tup: (i32, f64, u8) = (500, 6.4, 1);//Make a tuple
    let (x, y, z) = tup;                    //Divide the tuple into 3 vars, x, y, z
    println!("The value of y is: {}", x);   //Print whichever you want

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    
    //Prints all values in a tuple
    let i = 0;

    //Arrays
    //Iterate through (Crashes at the end)
    let days = ["Mon", "Tues", "Wed", "Thurs", "Fri", "Sat", "Sun"];
    let mut i = 0;
    loop {
        println!("{}", days[i]);
        i += 1;
    }
}
