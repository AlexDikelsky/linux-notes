use std::mem;

fn make_grid(size: usize) -> Vec<Vec<bool>> {
    let vec = vec![vec![false; size]; size];
    vec
}

fn print_grid(x: Vec<Vec<bool>>) {
    for i in x {
        for j in i {
            if j == false {
                print!("- ");
            }
            else {
                print!("X ");
            }
        }
        println!();
    }
}

fn fractal(tl: (u32, u32), size: u32, left: i8) -> Vec<(u32, u32)> {
    let mut empty = vec![];
    if left < 0 {
        return vec![tl];
    }
    else {
        empty.push(fractal((0, 0), size/2, left-1));
        empty.push(fractal((0, size/2), size/2, left-1));
        empty.push(fractal((size/2, 0), size/2, left-1)); //ADD LAST ONE LATER
        println!("{}", size);
        un_nest(empty)
    }
}

fn un_nest(vector: Vec<Vec<(u32, u32)>>) -> Vec<(u32, u32)> {
    let mut result = vec![];
    for i in vector {
        for j in i {
            result.push(j); 
        }
    }
    result
}

fn points_to_fill() -> Vec<(u32, u32)> {
    fractal((0, 0), 32, 4)
}

fn main() {
    let size = 32;
    
    let fill = points_to_fill();

    let mut grid = make_grid(size);
    for r in 0..size { 
        for c in 0..size {
            for point in &fill {
                if &(r as u32, c as u32) == point {
                    mem::replace(&mut grid[r][c], true);
                }
           }
        }
    }
    print_grid(grid);
}

    
