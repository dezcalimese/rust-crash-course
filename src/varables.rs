fn main() {
    // unsigned integer
    // u8, u16, u32, u64, u128
    let unsigned: u8 = 10;

    // signed integer
    // i8, i16, i32, i64, i128
    let signed: i8 = -10;

    // float is used for decimals
    let float: f32 = 1.2;

    println!("unsign: {} sign: {} float: {}", unsigned, signed, float);

    // char - can only be
    let letter = "c";
    let emoji = "\u{1F600}"; // :D

    println!("letter: {}, emoji: {}", letter, emoji);

    let is_true: bool = true;

    println!("isTrue: {}", is_true);
}
