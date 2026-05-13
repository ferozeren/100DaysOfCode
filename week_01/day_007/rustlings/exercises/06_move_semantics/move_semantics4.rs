fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn move_semantics4() {
        let mut x = Vec::new();
        let y = &mut x;
        y.push(42); // We can only have either mutliple reference or one mutable reference not both
        let z = &mut x;
        // y.push(42); // This will fail since the current mutable reference is z
        z.push(13);

        assert_eq!(x, [42, 13]);
    }
}
