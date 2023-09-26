use std::{thread, time};

fn main() {
    // Create a Duration of 20 seconds.
    let twenty_seconds = time::Duration::from_secs(20);

    // Put the current thread to sleep for 20 seconds.
    thread::sleep(twenty_seconds);

    // Print a message to the console.
    println!("Slept for 20 seconds!");
}
