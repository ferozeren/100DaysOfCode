// Booleans (`bool`)

fn main() {
    let is_morning = true;

    if is_morning {
        println!("Good morning!");
    }

    let is_evening = !is_morning; // If morning is true, it will be false else true
    if is_evening {
        println!("Good evening!");
    }

    let first = true;
    let second = false;

    if first != second {
        // Same as if !(first == second) {}
        println!("Ola");
    }
}
