mod mod_init;
use crate::mod_init::init;
mod mod_mesurements;
use crate::mod_mesurements::mesurements;
mod mod_commands;
use crate::mod_commands::commands;

fn main() {
    // get entries from file
    let mesurements = init::get_mesurements("mesurements");

    // loop through array and print each entry
    day1("day1", &mesurements);
    day2("day2", &mesurements);
    // Day 3
    let commands = init::get_file("commands");
    let position = commands::control(commands);
    println!("Day 3: {} is the position", position);
}

fn day1(label: &str, mesurements: &Vec<i32>) {
    let mut last_entry: &i32 = &mesurements[0];
    let mut counter: i32 = 0;
    mesurements.iter().for_each(|entry| {
        if entry > last_entry {
            counter += 1;
        }
        last_entry = entry
    });
    println!("{}: {} measurements are larger", label, counter);
}

fn day2(label: &str, measurements: &Vec<i32>) {
    let sum_chunks = mesurements::chunk_sum_measurements(measurements);
    day1(label, &sum_chunks);
}
