enum Command {
    Quit,
    Write(String),
    ChangeColor(i32, i32, i32),
    Move { x: i32, y: i32 },
}
impl Command {
    fn run(&self) -> String {
        match self {
            Self::Quit => "Quitting.".to_string(),
            Self::Write(message) => {
                println!("Message: {}", message);
                message.to_string()
            },
            Self::ChangeColor(_, _, _) => "Color changed.".to_string(),
            Self::Move {x: 0, y: 0} => "Didn't move.".to_string(),
            Self::Move {x, y} => format!("Moved: x={}, y={}.", x, y),
        }
    }
}

fn main() {
    let write_result = Command::Write(String::from("Hello!")).run();
    println!("Write result: {}", write_result);

    let change_color_result = Command::ChangeColor(255, 255, 255).run();
    println!("Change color result: {}", change_color_result);

    let move_1_result = Command::Move {x: 0, y: 0}.run();
    println!("Move 1 result: {}", move_1_result);

    let move_2_result = Command::Move {x: 0, y: 1}.run();
    println!("Move 2 result: {}", move_2_result);

    let quit_result = Command::Quit.run();
    println!("Quit result: {}", quit_result);
}
