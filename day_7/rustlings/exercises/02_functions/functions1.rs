// TODO: Add some function with the name `call_me` without arguments or a return value.

fn main() {
    call_me(); // function caller
}

// Simple function, returning a string slice.
fn call_me() -> &'static str {
    "Hello caller!" // Or return "Hello caller!", return is optional
}
