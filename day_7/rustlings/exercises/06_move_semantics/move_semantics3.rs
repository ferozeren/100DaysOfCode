// We don't need to create a variable to make moved data mutable
// We can declare vec as mutable in the function signature only.

fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(88);
    vec
}

fn fill_str_vec(mut vec: Vec<&'static str>, new_fruit: &'static str) -> Vec<&'static str> {
    vec.push(new_fruit);
    vec
}

fn main() {
    let vec_0 = vec!["Apple", "Orange", "Mango"];
    let vec_1 = fill_str_vec(vec_0, "Kiwi");

    println!("New Vec: {:#?}", vec_1); // We are using Debug trait here with :?, # is for pretty print
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics3() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}
