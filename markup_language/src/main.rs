#[derive(Copy, Clone)]

enum MachineState {
    Normal,
    Comment,
    Upper,
    Lower,
}

fn machine_cycle(state: MachineState, c: char) -> (Option<char>, MachineState) {
    use self::MachineState::*;
    match (state, c) {
        (Normal, '#') => (None, Comment),
        (Normal, '^') => (None, Upper),
        (Normal, '_') => (None, Lower),
        (Normal, other) => (Some(other), Normal),
        (Comment, '#') => (None, Normal),
        (Comment, _) => (None, Comment),
        (Upper, '^') => (None, Normal),
        (Upper, other) => (Some(other.to_ascii_uppercase()), Upper),
        (Lower, '_') => (None, Normal),
        (Lower, other) => (Some(other.to_ascii_lowercase()), Lower),
    }
}

fn main() {
    let mut state = MachineState::Normal;

    let mut processed_string = String::new();

    let input = "This _Is_ some ^input^. #we want this transformed without this comment#";

    for character in input.chars() {
        let (output, new_state) = machine_cycle(state, character);

        if let Some(c) = output {
            processed_string.push(c);
        }
        state = new_state;
    }

    println!("{}", processed_string);
}
