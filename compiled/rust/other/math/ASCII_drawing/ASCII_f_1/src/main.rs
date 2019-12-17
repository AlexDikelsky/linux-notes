use std::mem;
extern crate bresenham;
use bresenham::Bresenham;
mod read_nums;
mod sier;


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

fn main() {
    println!("What size do you want the grid to be?");
    //let length = read_nums::string_to_i8(read_nums::read_string());
    //let mut size = 1_usize;
    //for i in 0..length {
    //    size *= 2;
    //}

    let grid_size = 64;

    let fill = arbitrary::make(vec![(.5, .5, 0, 0, 0, 0));

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

    
