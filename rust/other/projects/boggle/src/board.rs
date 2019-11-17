use crate::SIZE;
use rand::Rng;

fn convert(x: usize) -> char {
    let letters = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];
    letters[x]
}

pub fn create() -> Vec<Vec<char>> {
    let mut rng = rand::thread_rng();
    let mut result: Vec<Vec<char>> = vec![vec!['a'; SIZE]; SIZE];
    
    let mut i = 0;
    let mut j = 0;

    while i < SIZE {
        while j < SIZE {
            result[i][j] = convert(rng.gen_range(0, 26));
            j += 1;
        }
        i += 1;
        j = 0;
    }

    println!("{}", SIZE);
    result
}
