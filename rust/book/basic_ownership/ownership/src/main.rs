fn main() {
    let s = String::from("hello");

    takes_ownership(s);

    //Only works if takes_ownership returns a value
    //println!("{}", takes_ownership(s));
    
    //Crashes because the String got blown up by going into the other function
    //println!("{}", s);
}

fn takes_ownership(some: String) {
    println!("{}", some)
}
