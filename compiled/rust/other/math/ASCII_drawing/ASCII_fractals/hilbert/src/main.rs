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
                print!("* ");
            }
        }
        println!();
    }
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


fn hilbert(x: isize, y: isize, size: isize, left: isize) -> Vec<(isize, isize)> {
    if left < 0 {
        return vec![(x, y)];
    }
    else {
        let x = vec![
                    hilbert(x, y,                   size/4, left-1), //Top left
                    hilbert(x/2+size/2, y,          size/4, left-1), //Top right
                    hilbert(x/2+size/2, y/2+size/2, size/4, left-1), //Bot left
                    hilbert(x, y/2+size/2,          size/4, left-1)  //Bot right
                    ];
        println!("{:?}", x);
        un_nest(x)
    }
}

fn main() {
    let size = 4;

    let fill = hilbert(0, 0, size, size);

    let mut grid = make_grid(size as usize);
    for r in 0..size { 
        for c in 0..size {
            for point in &fill {
                if &(r, c) == point {
                    mem::replace(&mut grid[r as usize][c as usize], true);
                }
            }
        }
    }
    print_grid(grid);
}

    
