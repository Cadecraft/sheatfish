// Imports
pub mod remdata;

/// Main function
fn main() {
    // Initialize REM, introductions
    let rem = remdata::RemData::new(
        "0.1.0",
        "2024/03/08",
        true
    );
    println!("SHEATFISH by Cadecraft");
    println!("{}", rem.fmt(false));
    println!("====");
    println!();

    // Initialize data
    // TODO

    // Start the command loop cycle
    // TODO
}
