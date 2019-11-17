use std::collections::HashMap;

fn count(v: &Vec<u32>) -> HashMap<u32, u32> {
    let mut used = HashMap::new();
    for ele in &v {
        if used.contains_key(ele) {
            used.insert(ele, used.get(ele) + 1);
        }
        else {
            used.insert(ele, 1);
        }
    }
    used
}

fn main() {
    let a = vec!(1, 1, 2, 2); 
    let b = vec!(2, 1, 1, 2); 
    let c = vec!(1, 2, 1, 2); 
    let d = vec!(2, 1, 2, 1); 
    let e = vec!(2, 2, 1, 1); 
    let f = vec!(1, 2, 2, 1); 

    let x = vec!(a, b, c, d, e, f);
    for ele in x {
        println!("It takes {} splits to split {:?} properly", 2, ele);
    }
}
