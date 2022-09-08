fn main() {
    let str: &str = "hello world";
    let mut string: String = String::from("Hello World");

    let slice = &string[..6]; // Get everything from index 0 up to not including index 6 itself
    slice.len();

    string.push('1'); // Allows you to add a character
    string.push_str("! Bob"); // Allows you to put in a string slice
    string = string.replace("Hello", "Bye");
    println!("{}", string);
}
