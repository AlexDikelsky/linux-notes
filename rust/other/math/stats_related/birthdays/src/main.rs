use rand::Rng;

fn main() {
    fill23();
}

fn fill23() -> [i32; 23] {  
    //Creates the struct
    let mut rng = rand::thread_rng();
    
    let mut people: [i32; 23];
    //let mut num: i32 = rng.gen_range(1, 366);

    /*Can't printlns over people here because all elements are uninitalized*/
    
    for i in 0..people.len() {
        //let num = rng.gen_range(1, 366);
        println!("{}", i);   
    }


    people
}


//Can't do this with arrays
//fn fill(len: i32) -> [i32; len] {
//    let i: i32 = rng.gen_range(1, 366);
//
//}
