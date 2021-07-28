// Loops - Used to iterate until a condition is met

pub fn run() {
    let mut count = 0;

    /*
    //Infinite Loop
    loop {
        count += 1;
        println!("Number: {}", count);

        if count == 5 {
            break;
        }
    }

    count = 1;
    //While Loop(Fizzbuzz)
    while count <= 100 {
        if count % 15 == 0 {
            println!("fizzbuzz");
        } else if count % 5 == 0 {
            println!("buzz");
        } else if count % 3 == 0 {
            println!("fizz");
        } else {
            println!("{}", count);
        }
        count += 1;
    }
    */

    //For loop
    for i in 0..101 {
        if i % 15 == 0 {
            println!("fizzbuzz");
        } else if i % 5 == 0 {
            println!("buzz");
        } else if i % 3 == 0 {
            println!("fizz");
        } else {
            println!("{}", i);
        }
    }
}