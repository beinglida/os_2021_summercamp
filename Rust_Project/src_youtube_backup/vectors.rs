// Vectors - Resizable arrays

use std::mem;

pub fn run() {
    let mut numbers: Vec<i8> = vec![1, 2, 3, 4, 5];

    //Re-assign value
    numbers[0] = 10;

    //Add on a vector
    numbers.push(6);
    numbers.push(7);

    //Pop off last value
    numbers.pop();
    numbers.pop();

    println!("{:?}", numbers);

    //Get single value
    println!("Single value: {}", numbers[0]);

    //Get vector length
    println!("Vector length: {}", numbers.len());

    //Vector are stack-allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    //Get slice
    let slice: &[i8] = &numbers[0..2];
    println!("Slice: {:?}", slice);

    //Loop through vector values
    for x in numbers.iter() {
        println!("Number : {} ", x);
    }

    //Loop and mutate values     iter_mut()
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Number Vec: {:?}", numbers);

}