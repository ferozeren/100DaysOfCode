fn call_me(num: u32) {
    // Functions args must have type annotation, just like const keyword
    for i in 0..num {
        // o..num is the range similar to python's range(0, num) or range(num)
        // we can make it inclusive by using =, for i in 0..=num {}
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    call_me(3);
}
