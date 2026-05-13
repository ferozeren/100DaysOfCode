fn animal_habitat(animal: &str) -> &str {
    // All return type must be same for identifier variable
    let identifier = if animal == "crab" {
        1
    } else if animal == "gopher" {
        2
    } else if animal == "snake" {
        3
    } else {
        u8::MAX // 4_i32, 4_u8, i32::MAX, etc are fine too, rust will infer it
                // as long as it's valid
    };

    // Here, based on the identifier variable we are returning &str to the function caller
    // remember we aren't using return since, its at the end of the fucntion, rust will
    // take last expression value  as return value
    if identifier == 1 {
        "Beach" // Same as return "Beach";
    } else if identifier == 2 {
        "Burrow"
    } else if identifier == 3 {
        "Desert"
    } else {
        "Unknown"
    }
}

fn main() {
    let student_average: f32 = 34.00;
    let user_grade: &str = {
        if student_average >= 95.0 {
            "S"
        } else if student_average >= 85.0 {
            "A"
        } else if student_average >= 70.0 {
            "B"
        } else if student_average >= 60.0 {
            "D"
        } else if student_average >= 50.0 {
            "E"
        } else if student_average >= 35.0 {
            "Passed"
        } else {
            "Failed"
        }
    };

    if ["S", "A", "B"].contains(&user_grade) {
        println!("Passed with Distinction");
    } else if ["C", "D", "E", "Passed"].contains(&user_grade) {
        println!("You did well, You Passed!");
    } else {
        println!("You didn't do well, You Failed!");
    }
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gopher_lives_in_burrow() {
        assert_eq!(animal_habitat("gopher"), "Burrow")
    }

    #[test]
    fn snake_lives_in_desert() {
        assert_eq!(animal_habitat("snake"), "Desert")
    }

    #[test]
    fn crab_lives_on_beach() {
        assert_eq!(animal_habitat("crab"), "Beach")
    }

    #[test]
    fn unknown_animal() {
        assert_eq!(animal_habitat("dinosaur"), "Unknown")
    }
}
