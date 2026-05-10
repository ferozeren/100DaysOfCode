// TODO: Fix the compiler error on this function.
fn picky_eater(food: &str) -> &str {
    if food == "strawberry" {
        // if `if` statement is true if block is executed else it will go down
        "Yummy!"
    } else if food == "potato" {
        // after `if` statemtn `else if` is check whether is true or false
        // if flase then its below condition willl be check else this block willl
        // executed

        "I guess I can eat that."
    } else {
        // else is the last blcok if all upper statment return false
        // then this block will be executed.
        "No thanks!"
    }
}

fn main() {
    let student_marks: [u32; _] = [90, 80, 83, 45, 35, 100, 98];
    let mut is_fail: bool = false;

    student_marks.iter().for_each(|mark| print!("{mark} "));
    println!();
    for mark in student_marks {
        if mark < 35 {
            is_fail = true;
            break;
        }
    }
    println!("Student: {}", if is_fail { "Failed" } else { "Passed" });
}

// TODO: Read the tests to understand the desired behavior.
// Make all tests pass without changing them.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yummy_food() {
        // This means that calling `picky_eater` with the argument "strawberry" should return "Yummy!".
        assert_eq!(picky_eater("strawberry"), "Yummy!");
    }

    #[test]
    fn neutral_food() {
        assert_eq!(picky_eater("potato"), "I guess I can eat that.");
    }

    #[test]
    fn default_disliked_food() {
        assert_eq!(picky_eater("broccoli"), "No thanks!");
        assert_eq!(picky_eater("gummy bears"), "No thanks!");
        assert_eq!(picky_eater("literally anything"), "No thanks!");
    }
}
