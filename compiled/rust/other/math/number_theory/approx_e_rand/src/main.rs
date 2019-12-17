use rand::prelude::*;
static REPS: usize = 10000;

fn main() {
    let mut rng = thread_rng();
    let mut total_reps = 0;

    for i in 0..REPS {
        let mut sum = 0.0;
        let mut times = 0;
        while sum < 1.0 {
            let rand_num: f64 = rng.gen();
            sum += rand_num;
            times += 1;
        }
        total_reps += times;
    }
    println!("{}", (total_reps as f64)/(REPS as f64));
}
