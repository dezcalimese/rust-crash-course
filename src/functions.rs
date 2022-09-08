fn main() {
    println!("{}", is_even(2));
}

// Visibility, function name (variable: variable type) -> return type
pub fn is_even(num: u8) -> bool {
    let digit: u8 = num % 2;
    digit == 0 // return bool
}
