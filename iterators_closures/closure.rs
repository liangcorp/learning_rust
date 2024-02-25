use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("Calculating Slowly...");
    thread::sleep(Duration::from_sec(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("Calculating Slowing...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!("Today do {} Pushups", expensive_closure(intensity));
        println!("Today do {} Situps", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Today take a break");
        } else {
            println!("Today run for {} minutes",
                                        expensive_closure(intensity));
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value,
                                    simulated_random_number);

}