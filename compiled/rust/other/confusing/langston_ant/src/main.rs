//Decided to do it in Python instead
fn main() {
   let mut a = Ant{location: [4, 5],  direction: 0};
   //println!("{:?}", a.get_location());
   //println!("{:?}", a.get_direction());

   for i in 0..10 {
//       if i > 5 {
//           print!("left: ");
//           println!("{}", a.get_direction());
//           a.go_left();
//       } else {
           print!("left: ");
           println!("{}", a.get_direction());
           a.go_left();
//       }
   }

}

struct Ant{
    location: [isize; 2],
    direction: usize,           
    //0 = up
    //1 = right
    //2 = down
    //3 = left
}

impl Ant{
    fn get_location(&self) -> [isize; 2] {
        self.location
    }
    fn get_direction(&self) -> usize {
        self.direction
    }

    fn go_right(&mut self) {
        // 0 -> 1
        // 1 -> 2
        // 2 -> 3
        // 3 -> 0
        self.direction = (self.direction + 1) % 4;
    }

    fn go_left(&mut self) {
        // 0 -> 3
        // 1 -> 0
        // 2 -> 1
        // 3 -> 2
        if self.direction == 0 {
            self.direction = 3;
        } else {
            self.direction -= 1;
        }
    }
}

