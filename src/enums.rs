fn main() {
    let a: MyEnum = MyEnum::A;
    let b: MyEnum = MyEnum::B(5);
    let c: MyEnum = MyEnum::C { x: 10, y: 20 };
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);

    // If our b type is an enum B, val represents our variable that we stored
    if let MyEnum::B(val) = b {
        println!("{}", val);
    }

    if let MyEnum::C { x, y } = c {
        println!("{} {}", x, y);
    }
}

#[derive(Debug)] // Tells compiler how to print enum
enum MyEnum {
    A,
    B(i32),
    C { x: i32, y: i32 }, // struct like object
}
