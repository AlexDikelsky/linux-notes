use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() {   
    //let f: &str = r"(1\d{3})";
    //println!("{:?}", perm(3, vec!['a', 'b', 'c']));
    try_regex();

}  

fn to_octal() -> io::Result<()> { // {{{
    let file = File::open("octal_nums.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        for ch in line.unwrap().chars() {
            let f = match ch {
                '0' => '0',
                '1' => '1',
                '2' => '+',
                '3' => '*',
                '4' => '.',
                '5' => '?',
                '6' => '\\',
                '7' => '|',
                _ => '?',
            };
            print!("{}", f);
        }
        println!();
    }
    Ok(())
} //}}}


fn print_nums(power: u32) -> () {
    for i in 0..(2_usize.pow(power)) {
        println!("{:0>4}", format!("{:b}", i));  
    }
}
// }}}
