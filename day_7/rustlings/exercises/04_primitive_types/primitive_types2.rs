// Characters (`char`)

fn main() {
    // Note the _single_ quotes, these are different from the double quotes
    // you've been seeing around.

    // Rust char is different compare to C's char

    let my_first_initial: char = 'C';
    if my_first_initial.is_alphabetic() {
        println!("Alphabetical!");
    } else if my_first_initial.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }

    let your_character = '🤓';

    if your_character.is_alphabetic() {
        println!("Alphabetical!");
    } else if your_character.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }

    println!("{}", "--".repeat(10));
    println!(
        "Charcter: {your_character}\nCharcter len\nUtf8: {}\nUtf16: {}",
        your_character.len_utf8(),
        your_character.len_utf16()
    );

    // In Rust, .len() returns the number of bytes, not characters. Since Rust
    // strings are UTF-8 encoded, simple ASCII characters like 'a' take 1 byte, but
    // emojis like '🤓' take 4 bytes. .chars().count() actually iterates through the
    // string to count the individual Unicode Scalar Values.

    println!("{}", "--".repeat(10));
    println!(
        "Char-&str: 🤓\nChar count = {}\nlen /size = {}",
        "🤓".chars().count(),
        "🤓".len()
    );
}
