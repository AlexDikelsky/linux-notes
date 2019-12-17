use rand::Rng; 
use std::collections::HashMap;

fn fill_vec(n:u32) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let mut v:Vec<u32> = Vec::new();
    let mut i = 0;

    while i < n {
        let num = rng.gen_range(1, 7); //Creates a random number between [1,7) in the integers
        v.push(num);
        i += 1;
    }
    v.sort();
    v
}

fn get_mean(v: &Vec<u32>) -> f64 {
    let mut sum = 0;
    for ele in v {
        sum += ele;
    }
    let sum_as_float: f64 = sum as f64;
    let length_as_float: f64 = v.len() as f64;

    sum_as_float / length_as_float
}

//Input a sorted vector
fn get_median(v: &Vec<u32>) -> f64 {
    match v.len() {
        1 => v[0] as f64,
        2 => get_mean(v),
        _ => get_median(&v[1..(v.len()-1)].to_vec()), //Works because not actually returning anything here
    }
}

fn get_mode(v: &Vec<u32>) -> Vec<u32> {
    let mut found = HashMap::new();
    for ele in v {
        found.insert(ele, 
                     match found.get(ele) {
                         None => 1,
                         Some(i) => i + 1,
                     });
    }
   
    let mut max = 0;
    let mut max_keys: Vec<u32> = Vec::new();

    for (key, value) in &found {
        if *value > max {
            max = *value;
            max_keys = vec![**key];
        }
        else if *value == max {
            max = *value;
            max_keys.push(**key);
        }
    }
    max_keys
}


fn main() {
    let rand_vector = fill_vec(10);

    let mode_vector = get_mode(&rand_vector);

    for ele in &rand_vector {
        println!("L = {}", ele);
    }
    println!("Mean = {}", get_mean(&rand_vector));
    println!("Median = {}", get_median(&rand_vector));
    print!("Mode Elements = ");
    for ele in &mode_vector {
        print!("{}, ", ele);
    }
    println!(); 
}
