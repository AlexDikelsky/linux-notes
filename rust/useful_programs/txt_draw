use std::mem;
extern crate bresenham;
use bresenham::Bresenham;

fn make_grid(size: usize) -> Vec<Vec<bool>> {
    let vec = vec![vec![false; size]; size];
    vec
}

fn print_grid(x: Vec<Vec<bool>>) {
    for i in x {
        for j in i {
            if j == false {
                print!("  ");
            }
            else {
                print!("* ");
            }
        }
        println!();
    }
}

fn bresen_vec(start: (isize, isize), end: (isize, isize)) -> Vec<(isize, isize)> {
    let line = Bresenham::new(start, end);
    let mut as_vec = vec![];
    for x in line {
        as_vec.push((x.0, x.1));
    }
    as_vec
}

fn un_nest(x: Vec<Vec<(isize, isize)>>) -> Vec<(isize, isize)> {
    let mut result = vec![];
    for i in x {
        for j in i {
            result.push(j);
        }
    }
    result
}

fn ser(x: isize, y: isize, size: isize, left: i8) -> Vec<(isize, isize)> {
    if left < 0 {
        return vec![(x, y)]
    }
    else {
        un_nest(vec![ser(x, y, size/2, left-1),
                    ser(x+size/2, y, size/2, left-1),
                    ser(x, y+size/2, size/2, left-1)])
    }
}

use std::io;

//Asks user for a string and returns it
fn read_string() -> String {
    let mut x = String::new();
    io::stdin().read_line(&mut x)
        .expect("Failed to read line");
    x    
}

//Takes a string and returns a u32. If can't be parsed, returns 0
fn string_to_i8(x: String) -> i8 {
    let x: i8 = match x.trim().parse() {
        Ok(num) => num,
        Err(_) => 0
    };
    x
}

fn main() {
    println!("What size do you want the triangle to be?");
    let length = string_to_i8(read_string());
    let mut size = 1_usize;
    for i in 0..length {
        size *= 2;
    }

    let fill = ser(0, 0, size as isize, length);

    let mut grid = make_grid(size as usize);
    for r in 0..size { 
        for c in 0..size {
            for point in &fill {
                if &(r as isize, c as isize) == point {
                    mem::replace(&mut grid[r][c], true);
                }
            }
        }
    }
    print_grid(grid);
}

    
