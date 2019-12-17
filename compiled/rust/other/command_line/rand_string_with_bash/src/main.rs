use std::io;

fn main() {
    //Takes in a random number, outputs a char, then exits
    let valid = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
    let choice: usize = string_to_usize(read_string());
    println!("{}", choice);
    println!("{}", valid[choice]);
}

//Asks user for a string and returns it
fn read_string() -> String {
    let mut x = String::new();
    io::stdin().read_line(&mut x)
        .expect("Failed to read line");
    x    
}
fn string_to_usize(x: String) -> usize {
    let x: usize = match x.trim().parse() {
        Ok(num) => num,
        Err(_) => 0
    };
    x
}

