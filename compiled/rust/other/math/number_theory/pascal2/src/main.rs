fn main() {
    //Generates 12 rows of pascal's triangle
    
    //print!("{}[2J", 27 as char);

    for j in 0..12 {
        print_prefix(j);
        for i in 0..j {
            print!("{}, ",n_choose_k(j, i));
        }
        print!("{}\n", 1);
    }
}

//n choose k formula (takes 2 ints and returns 1 int)
fn n_choose_k(n: u32, k: u32) -> u32 {
    fact(n)/(fact(k)*(fact(n-k)))
}

//given u32 returns the factorial as u32
fn fact(n: u32) -> u32 {
    if n <= 1 {
        1
    }
    else {
        fact(n - 1) * n
    }
}

//Prints the num in english formating
fn print_prefix(x: u32) -> () {
    if x == 1 {
        print!("1st: ");
    }
    else if x == 2 {
        print!("2nd: ");
    }
    else {
        print!("{}th: ", x);
    };
}
