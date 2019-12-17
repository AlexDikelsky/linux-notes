static SIZE: usize = 4;
mod board;
pub use crate::board::*;
fn main() {
    println!("{}", SIZE);
    println!("{:?}", board::create());
}
