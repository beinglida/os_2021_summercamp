// Enums are types which have a few definite values
enum movement {
    // Variants
    Up,
    Down,
    Left,
    Right,
}

fn move_avatar(m: movement) {
    match m {
        movement::Up => println!("Avatar moving Up"),
        movement::Down => println!("Avatar moving Down"),
        movement::Left => println!("Avatar moving Left"),
        movement::Right => println!("Avatar moving Right"),
    }
}

pub fn run() {
    let avatar1 = movement::Up;
    let avatar2 = movement::Down;
    let avatar3 = movement::Left;
    let avatar4 = movement::Right;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
}