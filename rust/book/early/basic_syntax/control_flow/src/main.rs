fn main() {
    let a = ["A", "B", "C", "D", "E"];

    //For each
    for element in a.iter() {
        println!("value = {}", element);
    }

    //In range (Reversed)
    for num in (1..4).rev() {
        println!("value = {}", num);
    }

    //In range (Normal)
    for num in 1..4 {
        println!("value = {}", num);
    }
}
