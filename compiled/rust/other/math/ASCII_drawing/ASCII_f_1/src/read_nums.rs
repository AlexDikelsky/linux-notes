use std::io;

//Asks user for a string and returns it
pub fn read_string() -> String {
    let mut x = String::new();
    io::stdin().read_line(&mut x)
        .expect("Failed to read line");
    x    
}

//Takes a string and returns a u32. If can't be parsed, returns 0
pub fn string_to_i8(x: String) -> i8 {
    let x: i8 = match x.trim().parse() {
        Ok(num) => num,
        Err(_) => 0
    };
    x
}

