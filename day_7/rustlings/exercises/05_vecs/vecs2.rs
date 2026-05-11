fn vec_loop(input: &[i32]) -> Vec<i32> {
    let mut output = Vec::new();

    // Dereferencing
    // for element in input {
    //     output.push(*element * 2) // * is being used here to derefer &i32 to i32
    // }

    // Or
    // Destructuring
    for &element in input {
        output.push(element * 2)
    }

    output
}

fn vec_map_example(input: &[i32]) -> Vec<i32> {
    // Auto Dereferencing, better make it obvious
    // input.iter().map(|element| element + 1).collect()

    // Referencing
    // input.iter().map(|element| *element + 1).collect() // Manual Deeferencing

    // Destructuring
    input.iter().map(|&element| element + 1).collect()
}

fn vec_map(input: &[i32]) -> Vec<i32> {
    // Dereferencing
    // input.iter().map(|element| *element * 2).collect()

    // Deestructuring
    input.iter().map(|&element| element * 2).collect() // Better
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let input = [2, 4, 6, 8, 10];
        let ans = vec_loop(&input);
        assert_eq!(ans, [4, 8, 12, 16, 20]);
    }

    #[test]
    fn test_vec_map_example() {
        let input = [1, 2, 3];
        let ans = vec_map_example(&input);
        assert_eq!(ans, [2, 3, 4]);
    }

    #[test]
    fn test_vec_map() {
        let input = [2, 4, 6, 8, 10];
        let ans = vec_map(&input);
        assert_eq!(ans, [4, 8, 12, 16, 20]);
    }
}
