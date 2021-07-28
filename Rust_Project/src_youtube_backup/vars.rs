pub fn run() {
    let name = "Brad";
    let mut age = 37;
    println!("My name is {}, I am {}", name, age);
    age = 38;
    println!("My name is {}, I am {}", name, age);

    //Define constants
    const ID: i32 = 001;
    println!("ID: {}", ID);

    //Assign multiple vars
    let (my_name, my_age) = ("Brad", 38);
    println!("{} is {}", my_name, my_age);

}