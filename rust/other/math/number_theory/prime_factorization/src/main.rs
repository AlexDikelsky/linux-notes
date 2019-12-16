fn factors(mut x: usize) -> Vec<usize> {
    let mut fact = vec![1]; 
    let mut comp = 2;
    while comp <= x {
        if x % comp == 0 {
            fact.push(comp);
            x /= comp;
        } else {
            comp += 1;
        }
    }
    fact
}

fn main() {
    for i in 1..150 {
        if {let mut prod = 1; //Gets the product of the factors
            for ele in factors(i) { prod *= ele; }
            prod} == i {
                println!("{}: {:?}", i, factors(i));
        } else {
            panic!("Failed for {}. vec={:?}", i, factors(i));
        }
    }
}
