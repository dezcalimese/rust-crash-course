use std::collections::HashMap;

#[derive(Debug)]
enum MyError {
    Error1
}

// Err, an enum that contains an error code and 
// Ok(value), A wrapper that contains a value 
fn divide(dividend: i32, divisor: i32) -> Result<i32, MyError> {
    if dividend % divisor !0 {
        Err(MyError::Error1) 
    } else {
        Ok(dividend / divisor)
    }
}

fn main() {
    let divide = divide(4, 2);
    // If there is an error, you can use `expect` function which prints statement if it gets an error type
    // let res = divide2.expect("we crashed");
    
    // One way of safely accessing types 
    match divide {
        Ok(v) => println!("{}", v),
        Err(v) => println!("{:?}", v)
    }

    // Another way of safely accessing types 
    /*      
    if divide.is_ok() {
        println!("{}", divide.unwrap());
    } 
    */

    // Directly unwrapping 
    // println!("{}", divide.unwrap());

    // `unwrap_or` function will unwrap the function and if error is returned, print out value  
    // println!("{}", divide.unwrap_or(100));
    // println!("{}", res)"
}