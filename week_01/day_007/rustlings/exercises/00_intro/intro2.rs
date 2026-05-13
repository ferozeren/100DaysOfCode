use std::io::Write;

fn main() -> std::io::Result<()> {
    println!("Hello world!");

    // Extra
    print!("Hello ");
    print!("World!");

    std::io::stdout().flush()?;
    // Same as
    match std::io::stdout().flush() {
        Ok(_) => (),
        Err(err) => return Err(err),
    }

    Ok(())
}
