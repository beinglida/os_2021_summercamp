// Arrays - Fixed list where elements are the same data types

use std::mem;

pub fn run() {
    let mut numbers: [i32; 4] = [1, 2, 3, 4];

    //Re-assign value
    numbers[0] = 10;

    println!("{:?}", numbers);

    //Get single value
    println!("Single value: {}", numbers[0]);

    //Get arrays length
    println!("Arrays length: {}", numbers.len());

    //Arrays are stack-allocated
    println!("Arrays occupies {} bytes", mem::size_of_val(&numbers));

    //Get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);
}