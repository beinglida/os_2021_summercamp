// macros4.rs
// Make me compile! Execute `rustlings hint macros4` for hints :)


macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };//add ;
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    };//add ;
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
