fn display_list(v: &Vec<u32>, n: usize) -> () {
     if v.len() == 0 {
         return
     }
     else if v.len() > n {
         display_line(&v[..n].into());
         println!();
         display_list(&v[n..].into(), n);
     }
     else {
         display_line(&v[..].into());
         return
     }
}

//Takes in one len=n slice and prints each element
fn display_line(v: &Vec<u32>) -> () {
    if v.len() == 0 {
        return
    }
    else {
        print!("{}  ", v[0]);
        display_line(&v[1..].into());
    }
}

fn main() {
    let mut v: Vec<u32> = Vec::new();
    let c = 3;
    for i in 0..6{
        v.push(i);
        display_list(&v, c);
        println!("\n--------------------------------");
    }
}
