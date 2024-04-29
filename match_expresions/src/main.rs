enum Action {
    Drive,
    Pickup,
    Turn(Direction),
    Stop,
}

enum Direction {
    Left,
    Right,
}

fn print_action(a: &Action) {
    match a {
        Action::Drive => println!("Drive Forward!"),
        Action::Turn(direction) => match direction {
            Direction::Left => println!("Turn Left"),
            Direction::Right => println!("Turn Right"),
        },
        Action::Pickup => println!("Pickup object!"),
        Action::Stop => println!("Stop"),
    }
}

fn main() {
    // use self::Action::*;
    // use self::Direction::*;
    let program = vec![
        Action::Drive,
        Action::Turn(Direction::Left),
        Action::Drive,
        Action::Pickup,
        Action::Drive,
        Action::Turn(Direction::Left),
        Action::Turn(Direction::Left),
        Action::Turn(Direction::Left),
        Action::Turn(Direction::Left),
        Action::Drive,
        Action::Turn(Direction::Right),
        Action::Drive,
        Action::Stop,
    ];

    for action in program {
        print_action(&action);
    }
}
