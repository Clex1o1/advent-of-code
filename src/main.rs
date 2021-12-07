mod mod_init;
pub use crate::mod_init::init;

fn main() {
    // get entries from file
    let mesurements = init::get_mesurements();
    // loop through array and print each entry
    let mut last_entry: &i32 = &mesurements[0];
    let mut counter: i32 = 0;
    mesurements.iter().for_each(|entry| {
        if entry > last_entry {
            counter += 1;
        }
        last_entry = entry
    });
    println!("{} measurements are larger", counter);
    println!("{} measurements total", mesurements.len());
}
