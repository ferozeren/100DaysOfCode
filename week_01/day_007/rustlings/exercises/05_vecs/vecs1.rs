fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // Array

    let v = Vec::from(a); // Best and performant

    // let v = a.to_vec(); // Identitcal performance

    // let v = vec![10, 20, 30, 40]; // If you create a vec from scratch

    // let v = a.to_owned() // This will fail
    // let v = a[..].to_owned(); // Conver the slice into Owned Vec

    // let mut v = Vec::with_capacity(a.len());
    // a.into_iter().for_each(|val| v.push(val)); // Use this only if you need to transform the data // (e.g., v.push(val * 2))
    // Its Same as
    // for val in a {
    //     v.push(val)
    // }

    (a, v)
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, *v);
    }
}
