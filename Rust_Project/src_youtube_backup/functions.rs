// Functions - Used to store blocks of code for re-use

pub fn run() {

    //Bind functions values to variables
    greeting("Hello", "Bea");
    let get_sum = add(2, 3);
    println!("Sum: {}", get_sum);

    //Closures
    let n3: i32 = 10;
    let add_sum = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("add_sum: {}", add_sum(1, 2));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}