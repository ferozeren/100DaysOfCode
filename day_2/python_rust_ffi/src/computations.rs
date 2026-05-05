use std::time::Duration;

pub fn compute() {
    println!("Computing...");
    std::thread::sleep(Duration::from_secs(10));
    println!("Done 😀.")
}
