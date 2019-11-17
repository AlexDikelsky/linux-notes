use std::env;
use std::fs;


fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, filename) = parse_config(&args);
}

fn parse_config(args: &[String]) ->(&str, &str) {
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}

fn first_attempt() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let filename = &args[2];
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    println!("query={}, filename={}", query, filename);
    println!("With text:\n{}", contents);
}

fn count_slashes(s: &String) -> u8 {
    let mut i = 0;
    for ele in s.chars() {
        if ele == '/' {
            i+=1;
        }
    }
    i
}   
