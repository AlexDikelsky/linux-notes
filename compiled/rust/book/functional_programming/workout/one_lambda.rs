use std::thread;
use std::time::Duration;


fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_closure(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_closure(intensity)
        );
    }
    else {
        if random_number == 3 {
            println!("Take a break today!");
        }
        else {
            println!("Today, run for {} minutes!",
            expensive_closure(intensity));
        }
    }
}

fn main() {
    let sim_user_choosen = 10;
    let sim_random_number = 7;

    generate_workout(
        sim_user_choosen,
        sim_random_number,
    );
}
