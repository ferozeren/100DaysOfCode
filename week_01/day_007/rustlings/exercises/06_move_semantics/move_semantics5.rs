#![allow(clippy::ptr_arg)]

// Shouldn't take ownership, we are borrowing
fn get_char(data: &str) -> char {
    // &str is same as &String mostly
    data.chars().last().unwrap()
}

// Should take ownership, moving
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();
    println!("{data}"); // Note println! macro always bro unlike dbg! macro will take owernship
    dbg!(data); // We can use data from this since its consumed by dbg! macro
}

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data); // We need to pass as reference since we need to use data for string_uppercase to.

    string_uppercase(data);
}
