use std::io;

fn main() {
    println!("Type 2 integers");

    let a: u32 = string_to_u32(read_string());
    let b: u32 = string_to_u32(read_string());

    let inter: f64 = (a*a + b*b).into();
    
    let result = f64::sqrt(inter) == f64::floor(f64::sqrt(inter));

    match result {
        true => println!("Yes, {} + {} is a Pythagorean Triple", a, b),
        false => println!("No, {} + {} is not a Pythagorean Triple", a, b),
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

