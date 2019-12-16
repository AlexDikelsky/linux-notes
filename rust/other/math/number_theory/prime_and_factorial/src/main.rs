fn fact(n: u64) -> u64 {
   if n <= 1 {
       1
   }
   else {
       fact(n-1) * n
   }
}

fn is_prime(n: u64) -> bool {
   for i in 2..n {
       if n % i == 0 {
           return true
       }
   }
   false
}

fn main() {
    println!("{}, 8", is_prime(8));
    println!("{}, 9", is_prime(9));
    println!("{}, 2", is_prime(2));
    println!("{}, 7", is_prime(7));
    println!("{}", fact(5));
}
