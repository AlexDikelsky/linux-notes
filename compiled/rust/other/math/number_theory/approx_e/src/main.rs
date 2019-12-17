fn main() {
    let mut f: f64 = 1.0; 
    let mut sum: f64 = 0.0;
    let mut duration = 0;
    while duration < 10 {
        sum += 1.0/f;
        println!("{}", sum);
        
        //Multiply it by itself +1 so you don't actually need to calculate factorial a bunch
        f = f*(f+1.0);
        duration += 1;
    }
}
