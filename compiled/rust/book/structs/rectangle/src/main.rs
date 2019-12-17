//#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
    name: String,
}

impl Rectangle {
    //Constructs a square with a given size
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size, name: "Henry".to_string() }
    }

    //Finds the area of the rectangle
    fn area(&self) -> u32 {
        self.width * self.height
    }

    //Finds if a rectangle can hold another rectangle
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    //Prints "hello [name]"
    fn hello_shape(&self) -> () {
        println!("Hello, {}", self.name);
    }
}


fn main() {
    //Constructing by just giving values
    let rect1 = Rectangle { width: 30, height: 50, name: "Jeff".to_string() };
    //Constructing with square constructor
    let sq = Rectangle::square(100);

    //Only using self as args
    rect1.hello_shape();
    sq.hello_shape();

    //Also only using self as args, but returning a value to print
    println!("{}", rect1.area());
    println!("{}", sq.area());

    //Have to only use a reference of rect1
    println!("sq can hold rect1? {}", sq.can_hold(&rect1));
    //Dosen't change ownership of sq because sq wasn't passed anywhere
    println!("sq can hold rect1? {}", sq.can_hold(&rect1));
    println!("rect1 can hold sq? {}", rect1.can_hold(&sq));
}
