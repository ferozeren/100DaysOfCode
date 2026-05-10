fn call_me(num: u8) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    // TODO: Fix the function call.
    call_me(u8::MAX); // We need to provide parameter defined in fuction signature.
                      // u*::MAX is the max value u8 can hold i.e., 255 (0..=255)
}
