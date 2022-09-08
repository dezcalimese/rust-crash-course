fn main() {
    let arr: [u8; 3] = [1, 2, 3];
    // You can also include number you want to populate array with and give it number to represent the length of the array you want to create
    let other_arr: [u8; 5] = [100; 5];

    println!("index: {}, length: {}", arr[0], other_arr.len());

    // print structure of array and other objects
    println("{:?}", other_arr);
}
