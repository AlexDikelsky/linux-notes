use std::io;

fn main() {
    println!("Type a postitive integer");
    let a = string_to_u32(read_string());    
    println!("{} is the {}th number in the Fibbanachi Sequence", calc_fib(a), a); 
}

fn calc_fib(x: u32) -> u32 {
    if x <= 1 {
        x
    }
    else {
        calc_fib(x-2)+calc_fib(x-1)
    }
}

//Asks user for a string and returns it
fn read_string() -> String {
    let mut x = String::new();
    io::stdin().read_line(&mut x)
        .expect("Failed to read line");
    x    
}

//Takes a string and returns a u32. If can't be parsed, returns 0
fn string_to_u32(x: String) -> u32 {
    let x: u32 = match x.trim().parse() {
        Ok(num) => num,
        Err(_) => 0
    };
    x
}

