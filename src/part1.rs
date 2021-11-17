// Part 1
// This part includes practice problems for learning Rust on Varibles and Functions



// Hu Tutu is buying apples. One apple usually costs 2 Rustbucks, but if you buy
// more than 40 at once, each apple only costs 1! Write a function that calculates
// the price of an order of apples given the quantity bought. 
fn calculate_apple_price(amount: i32) -> i32{
    print!("{}",amount);
    unimplemented!();
}


#[test]
fn part1_test() {
    let price1 = calculate_apple_price(35);
    let price2 = calculate_apple_price(40);
    let price3 = calculate_apple_price(65);

    assert_eq!(70, price1);
    assert_eq!(80, price2);
    assert_eq!(65, price3);
}