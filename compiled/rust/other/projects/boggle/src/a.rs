    static SIZE: usize = 5;
    extern crate rand;
    
    fn convert(x: usize) -> char {
        let letters = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];
        letters[x]
    }
    
    pub fn create() -> Vec<Vec<char>> {
        let mut rng = rand::thread_rng();
        let mut result: Vec<Vec<char>> = vec![vec![convert(rng.gen_range(0, 25)); SIZE]; SIZE];

        result
    }
    
    fn main() {
        println!("{:?}", create());
    }
