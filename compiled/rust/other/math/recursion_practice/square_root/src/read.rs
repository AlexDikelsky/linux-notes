use std::io;

pub fn read_f64() -> f64 {
    string_to_f64(read_string()) 
}

//Asks user for a string and returns it
fn read_string() -> String {
    let mut x = String::new();
    io::stdin().read_line(&mut x)
        .expect("Failed to read line");
    x    
}

//Takes a string and returns a u32. If can't be parsed, returns 0
fn string_to_f64(x: String) -> f64 {
    let x: f64 = match x.trim().parse() {
        Ok(num) => num,
        Err(_) => 0.0
    };
    x
}
