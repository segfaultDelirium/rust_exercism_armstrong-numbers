// assumes c is within '0'..='9'
fn char_to_digit(c: char) -> u8 {
    c as u8 - '0' as u8
}

pub fn is_armstrong_number(num: u32) -> bool {
    // todo!("true if {num} is an armstrong number")
    let digits: Vec<u8> = num.to_string().chars().map(|c| char_to_digit(c)).collect();
    let exponent = digits.len();

    let sum: u64 = digits
        .into_iter()
        .map(|d| (d as u64).pow(exponent as u32))
        .sum();
    sum == num as u64

    // false
}
