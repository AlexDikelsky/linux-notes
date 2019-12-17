use rand::Rng; 
//Add 
//rand = "0.6"
//to Cargo.toml

fn fill_vec(n:u32) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let mut v:Vec<u32> = Vec::new();
    let mut i = 0;

    while i < n {
        let num = rng.gen_range(1, 7); //Creates a random number between [1,7) in the integers
        v.push(num);
        i += 1;
    }
    v
}

fn main() {
    for ele in fill_vec(9) {
        println!("{}", ele);
    }
}

