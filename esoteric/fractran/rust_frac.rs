fn main() {
    let a = vec![17, 78, 19, 23, 29, 77, 95, 77,  1, 11, 13, 15, 1, 55];
    let b = vec![91, 85, 51, 38, 33, 29, 23, 19, 17, 13, 11,  2, 7,  1];
    let n = 2u64;
    let max_runs = 1000;
    let e = run(n, a, b, max_runs);
    only_powers_of_2(e, 20);
}

fn is_integral(N : u64, a: u64, b: u64) -> bool {
    //You can't reduce this to 
    //N % b or a % b
    //because if 
    //2 % 4 or 2 % 4
    //would return false, though it should be true
    return (N * a) % b == 0

}

fn only_powers_of_2(integers: Vec<u64>, max_pow: usize)  {
    let powers: Vec<u64> = (0..(max_pow)).map(|r| (2_u64.pow(r as u32) as u64)).collect();
    let a: Vec<&u64> = integers.iter().filter(|x| powers.contains(x)).collect();
    println!("{:?}", a);
}

fn run(mut N: u64, numer: Vec<u64>, denom: Vec<u64>, upper_bound: u64) -> Vec<u64> {
    let mut n_values = vec![];
    let mut runs = 0;

    while runs < upper_bound {
        let mut i = 0;
        let mut found = false;
        while i < numer.len() && !found {
            if is_integral(N, numer[i], denom[i]) {
                //println!("{}, {}", N, runs);
                n_values.push(N);
                N = (N * numer[i]) / denom[i];
                found = true;
            }
            i += 1;
        }
        runs += 1;
    }
    return n_values
}



