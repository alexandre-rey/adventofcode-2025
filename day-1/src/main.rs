use std::fs;

enum Direction {
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err("invalid direction"),
        }
    }
}

struct CounterState {
    zero_count: i32,
    counter: i32,
}

impl CounterState {
    fn new(start: i32) -> Self {
        Self {
            counter: start,
            zero_count: 0,
        }
    }

    fn apply_move(&mut self, direction: Direction, count: i32) {
        const RING_SIZE: i32 = 100;

        for _ in 0..count {
            match direction {
                Direction::Left => {
                    self.counter -= 1;
                    if self.counter < 0 {
                        self.counter = RING_SIZE - 1;
                    }
                }
                Direction::Right => {
                    self.counter += 1;
                    if self.counter >= RING_SIZE {
                        self.counter = 0;
                    }
                }
            }

            if self.counter == 0 {
                self.zero_count += 1;
            }
        }
    }
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let mut state = CounterState::new(50);

    for (index, cmd) in input.lines().enumerate() {
        if cmd.is_empty() {
            continue;
        }

        let direction = cmd.chars().next().unwrap_or('E');
        let count = cmd[1..].parse().expect("Invalid count value");

        match direction {
            'R' | 'L' => {
                let direction = Direction::try_from(direction).unwrap();
                state.apply_move(direction, count)
            }
            'E' => println!("Finish"),
            other => panic!("Unknown char: {other}"),
        }

        println!("{index} : {}", state.counter);
    }

    println!("Zero counter : {}", state.zero_count);
}
