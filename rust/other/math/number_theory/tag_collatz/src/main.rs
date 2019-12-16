//Take in an integer n
//Convert it to a string of As with len n
//
//if first letter = A
//  take out first 2 letters
//  append BC
//
//if first letter = B
//  take out first 2 letters
//  append A
//
//if first letter = C
//  take out first 2 letters
//  append AAA

use std::io;

fn main() {
    println!("Pick a number");
    let n = string_to_u32(read_string());
    let mut text = make_inital_string(n);
    
    //Applys the tag rule until it reaches "A"
    while text != String::from("A") {
        println!("{}", text);
        text = apply_tag(text);
    }
    println!("A");
}

//Does the if statments at the top of the program given a string
fn apply_tag(mut text: String) -> String {
    let first_char = text.chars().next().unwrap();
    text = text[2..].to_string();

    match first_char {
        'A' => text.push_str("BC"),
        'B' => text.push_str("A"),
        'C' => text.push_str("AAA"),
        _ => text = String::from("A"),
    }
    text
}

//Takes an integer, returns a string of As with given len
fn make_inital_string(n: u32) -> String {
    let mut text = String::new();
    for i in 0..n {
        text.push('A');
    };
    text
}

//Asks user for a string and returns it
fn read_string() -> String {
    let mut x = String::new();
    io::stdin().read_line(&mut x)
        .expect("Failed to read line");
    x    
}

//Takes a string and returns a u32. If can't be parsed, returns 1
fn string_to_u32(x: String) -> u32 {
    let x: u32 = match x.trim().parse() {
        Ok(num) => num,
        Err(_) => 1
    };
    x
}

