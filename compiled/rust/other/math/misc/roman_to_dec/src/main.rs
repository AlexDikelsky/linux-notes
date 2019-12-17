fn main() {
    println!("{}", roman_to_int("MCMXCIV".to_string()));
}

pub fn val(c: char) -> i32 {
    match c {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        _ => 0,
    }
}

pub fn roman_to_int(s: String) -> i32 {
    let mut prev = 'A';
    let mut sum = 0;

    for c in s.chars() {
        if val(prev) < val(c) {
            sum -= val(prev);
        } else {
            sum += val(prev);
        }
        prev = c;
    }
    sum + val(prev)
}

