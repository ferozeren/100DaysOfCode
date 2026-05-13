// Mary is buying apples. The price of an apple is calculated as follows:
// - An apple costs 2 rustbucks.
// - However, if Mary buys more than 40 apples, the price of each apple in the
// entire order is reduced to only 1 rustbuck!

fn calculate_price_of_apples(apple_quantity: u32) -> u32 {
    let apple_price = if apple_quantity > 40 { 1 } else { 2 };
    apple_quantity * apple_price

    // Same as
    // if apple_quantity > 40 {
    //     apple_quantity
    // } else {
    //     apple_quantity * 2
    // }
}

fn calcualte_discount_amount(total_amount: f64, is_black_friday_sale: bool) -> f64 {
    let discount_rate = match is_black_friday_sale {
        true => 0.15,
        false => 0.05,
    };

    // Same as
    // let discount_rate = if is_black_friday_sale { 0.15 } else { 0.05 };

    total_amount * discount_rate
}

fn main() {
    let current_month = "Nov";
    let is_black_friday_sale = { current_month == "Nov" };

    let total_amount = 450_000.0;
    let after_discount =
        total_amount - calcualte_discount_amount(total_amount, is_black_friday_sale);
    println!("Before Discount: {total_amount}\nAfter Discount : {after_discount} ")
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_test() {
        assert_eq!(calculate_price_of_apples(35), 70);
        assert_eq!(calculate_price_of_apples(40), 80);
        assert_eq!(calculate_price_of_apples(41), 41);
        assert_eq!(calculate_price_of_apples(65), 65);
    }
}
