enum Movement {
    Up,
    Down,
    Right,
    Left,
}
fn move_avatar(g:Movement) {
    match g {
        Movement::Up => println!("Avatar moves upward"),
        Movement::Down => println!("Avatar moves downward"),
        Movement::Right => println!("Avatar moves right"),
        Movement::Left => println!("Avatar moves left"),
    }
}

pub fn run() {
    let avatar1 = Movement::Down;
    let avatar2 = Movement::Up;
    let avatar3 = Movement::Right;
    let avatar4 = Movement::Left;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
}