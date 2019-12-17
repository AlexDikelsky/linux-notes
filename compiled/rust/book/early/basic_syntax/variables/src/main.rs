fn main() {

    /* Doesn't work because x is a constant
    let x=5;
    x=7;
    */

    /* Works because x is mutable
    let mut x=5;
    println!("The value of x is {}", x);
    x=7;
    println!("The value of x is {}", x);
    */
    
    //Makes a const variable
    //const MAX_POINTS: u32 = 100_000;
    //println!("{}", MAX_POINTS);
    
    /*Shadowing
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is {}", x);
    */

    /* Works because of the let
    let spaces = "   ";
    let spaces = spaces.len();
    println!("Num spaces is {}", spaces);*/

    /* Doesn't work because converting a string variable to an integer variable
    let mut spaces = "   ";
    spaces = "A";
    let mut spaces = "   ";
    spaces = spaces.len();*/
}
