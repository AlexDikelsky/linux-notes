mod read;
use read::*;

static MIN_ERROR: f64= 0.00000000000001;

fn square_root(target: f64) -> f64 { 
    if target < 1.0 && target != 0.0 { //If it's less than 1, invert the fraction, and find the inverse's square root, then return the inverse
        let x = get_better_guess(0.0, 1.0/target, 1.0/target/2.0, 1.0/target);
        1.0/x
    } else {
        get_better_guess(0.0, target, target/2.0, target)
    }
}

fn get_better_guess(low: f64, high: f64, guess: f64, target: f64) -> f64 {
    //println!("l={}, h={}, g={}, t={}", low, high, guess, target);
    let squared = guess * guess;
    if (squared-target).abs() < MIN_ERROR || low == guess || high == guess {
        guess
    } else if squared < target {
        get_better_guess(guess, high, (guess-low).abs()/2.0+guess, target) //the whole guess thing returns the 'midpoint' between the (high or low) and guess
    } else if squared > target {
        get_better_guess(low, guess, (guess-high).abs()/2.0+low, target)
    } else {
        panic!("Somehow the equals thing didnt work. squared = {}, target = {}.", squared, target);
    }
}

fn main() {
    let mut valid = false;
    println!("Pick a number to square root");
    while !valid {
        let x=read::read_f64();
        println!("Pick a number: {}", x);
        if x < 0.0 {
            println!("No real solutions");
        } else {
            println!("{}", square_root(x));
            valid = true;
        }
    }
}
