fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1_i32, 2, 3, 4, 5];

        let nice_slice = &a[1..4]; // Same as &a[1..=3], use inclusive in cases like x..=255, the max limit of u8

        assert_eq!([2, 3, 4], nice_slice);
    }
}
