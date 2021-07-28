// macros2.rs
// Make me compile! Execute `rustlings hint macros2` for hints :)

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

//macro_rules! my_macro 和fn main()交换次序

fn main() {
    my_macro!();
}