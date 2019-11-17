use std::io;

fn main() {
    //Generates given row of pascal's triangle
    
    print!("{}[2J", 27 as char);

    println!("Type the level of Pascal's Triangle you want to see");

    let a = string_to_u32(read_string());

    for i in 0..(a) {
        print!("{}, ",n_choose_k(a, i));
    }
    print!("{}\n", 1);
}

fn n_choose_k(n: u32, k: u32) -> u32 {
    fact(n)/(fact(k)*(fact(n-k)))
}

fn fact(n: u32) -> u32 {
    if n <= 1 {
        1
    }
    else {
        fact(n - 1) * n
    }
}

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

