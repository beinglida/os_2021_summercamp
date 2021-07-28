pub fn run() {
    //Print to console
    println!("Hello from the print.rs file");

    //Basic Formatting
    println!("{} is from {}", "Brad", "Mass");

    //Positional Arguments
    println!("{0} is from {1}, {0} likes to {2}", "Brad", "Mass", "code");

    //Name Arguements
    println!("{name} likes playing {activity}", name = "Sheldon", activity = "Golf");

    //Placeholders Traits
    println!("Binary:{:b}, Octal: {:o}, Hex: {:x}", 10, 10, 10);

    //Placeholders for debug trait
    println!("{:?}", (12, true, "hello"));

    //Basic Mathe
    println!("10 + 10 = {}", 10 + 10);

}