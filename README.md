# Rust Crash Course for Solana

## Using Cargo 
- To compile & execute Rust code at the same time, run `cargo run` 

## Printing to the console
- Macros have the exclamation mark (ex: `println!` prints out the string given and creates a new line)
- If you want to include variables, you need to add curly braces 
  - These curly braces represent a variable 
- The macro takes in the string and automatically makes sure that there are an exact amount of variables passed that correspond with the curly braces
- Ex:
```rust
fn main() {
    let a = 10;
    let b = 15;
    println!("Hello, world!, {} {}", a, b);
}
```

## Variables: Scalar Types
- 4 Scalar types: integers, floats, characters, and booleans
- 2 types of integers: unsigned and signed integers
  - Unsigned: can't be negative 
- Floats represent decimals
- Characters, or chars, can be one character of a string 
  - Rust supports unicode 

## Variables: Arrays
- You can create an array either how you normally would in other languages, and you can also include the number you want to populate array with and give it a number to represent the length of the array you want to create 
- This syntax lets us print out the whole structure of an array `{:?}`

## Variables: Tuples
- In Rust, a tuple can hold an unlimited amount of elements, and can hold different types together 
- You can let Rust figure out the types as well and not even define them
- To access the elements in tuples, you use dots and the index you want, ex: `tuple.0`
- Destructuring is extracting the parameters inside your object into individual variables 

## Functions 
- By default, all Rust functions are private unless you specifically set it to public (`pub`)
- To signify you're returning a variable, you simply write your statement and not include the semicolon
  - In all instances, you need a semicolon to end your statements unless it's the return value of a function

## Mutability 
- Rust enforces variable mutability, which prevents you from being able to modify the data type
- If you want to change the variable, you just use the keyword `mut` in front of the variable 
  - This tells the compiler that you want to change your variable throughout your code.

## Arrays + Slices
- A slice is a subarray 
  - The first value is inclusive and the last value is exclusive
- To get a pointer to our subarray, we need 
- For arrays, at compile time we know the length; however for slices we don't
- On Solana, a lot of the data passed through from the frontend comes through a slice 

## Strings
- The type of `str` is `&str`, which is a string literal which is a reference to a space in memory holding the constant `"<Insert text here>"`
- To create a string, we can use the type `String` which is actually an object 
  - Two colons are used to reference a function 

## If Statements
- Similar to Python style, typical setup & syntax

## For loop
- Also very similar to Python
- In Rust, the for loop always traverses through an array we generate as opposed to other languages

## While loop
- Similar to other languages
- If you have a `break` statement, you don't need to add a semicolon after

## Match statement
- This is similar to a switch statement in other languages
  - Give parameter that you want to match and find specific cases you want to match + code you want to execute with the case

## Structs
- Basically a class 
- To implement methods into your struct, you use the keyword `impl` and give the struct you want to add the implementation for
  - You have to define your `self` in the function, similar to Python
  - Every function in the implementation will have a pointer too
- If the parameter you're trying to use is the same as the name of the field you're trying to populate, you can shorten everything and just pass in the variable  

## Traits
- Rust doesn't support inheritance, it only suppports interfaces
- Traits are used to handle interfaces
- You can define a trait by using the keyword `trait` and giving the name of the trait you want other classes to implement 
- You can create a function that has been implemented by the class that's extending it 
  - They can either not take parameters and reference themselves or add default implementations 
  - These implementations can be overriden when extended 
- To extend a trait, we can use `impl` and implement the trait for the struct 

## Enums 
- Enums are objects that represent a certain value; however in Rust additional values can be stored 
- To create an enum, you call the class you created then double colon to reference the field ex: `let a: MyEnum = MyEnum::A;`
- One way to extract the value of enums is through an if statement

## Vectors
- A vector is a dynamic array
- To create a vector, use the keyword `vec!` which is a macro Rust provides us  
- We can initialize it the same way we do arrays
  - Can define the value in each index, or provide a range
- A vector's size can be dynamically altered 

## Hash Maps
- Objects that allow you to store data in a key value pairing
- You have to import them from the standard library 
- Getting a map value creates an option type, which is basically an enum with two values: Some or None (success or failure type)

## Options 
- As aforementioned, options are enums which have two types: None (which will throw an exception) and also a Some type which stores a value
- There are a couple ways of extracting the value of an option 
  - Using a match statement to get the value
  - Calling an unwrap function on the code  

## Results
- Results are similar to options, but have an Ok and an Err type 
  - Error types can be anything 
  - Ok types contain types that are returned when successful
- One way of safely accessing the type is using a match statement
  - We could also use an if statement 