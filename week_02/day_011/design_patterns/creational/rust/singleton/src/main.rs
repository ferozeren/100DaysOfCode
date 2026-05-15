use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref ARRAY: Mutex<Vec<u8>> = Mutex::new(vec![]);
}

fn call() {
    ARRAY.lock().unwrap().push(1);
}

fn main() {
    call();
    call();
    call();

    println!("Called {}", ARRAY.lock().unwrap().len());
}
