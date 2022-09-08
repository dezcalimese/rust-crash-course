fn main() {
    let i = 5;
    match i {
        0 => println!("0"),
        1 | 2 => println!("1,2"),
        3..=4 => println!("3,4"), // this is a range, equals sign at 4 makes it inclusive
        _ => println!("default"), // if none of the cases match, this is catchall case
    }
}
