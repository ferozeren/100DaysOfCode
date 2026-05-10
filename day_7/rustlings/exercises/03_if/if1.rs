fn bigger(a: i32, b: i32) -> i32 {
    if a > b {
        // If this condition is true, proceed with if block
        // else go to else block
        a
    } else {
        // Lese block will be execute if the if condition is false
        b
    }
}

fn main() {
    // You can optionally experiment here.

    let my_name: &str = "ferozeren";

    if !my_name.is_empty() {
        // Same as if !(my_name.len() > 0)  {}
        // '!` will make statment opposite, true -> false, false -> true.`
        println!("Hello, {}", my_name); // Same as println!("Hello, {my_name}");
    } else {
        println!("Empty name is provided!")
    }
}

// Don't mind this for now :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }

    #[test]
    fn equal_numbers() {
        assert_eq!(42, bigger(42, 42));
    }
}
