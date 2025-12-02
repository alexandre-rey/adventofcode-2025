struct Range {
    start: i64,
    end: i64,
}

pub struct Part1 {
    input: String,
}

impl Part1 {
    pub fn new(input_str: &str) -> Self {
        Self {
            input: input_str.to_string(),
        }
    }

    pub fn run(&self) {
        let inputs = self.parse_input();
        let mut result: Vec<i64> = Vec::new();
        for r in inputs {
            result.append(&mut self.split_range(r));
        }

        let mut final_solution = 0;
        for solution in result {
            final_solution += solution;
        }

        println!("Part 1 final solution: {}", final_solution);
    }

    fn split_range(&self, range: Range) -> Vec<i64> {
        let mut result = Vec::new();
        for i in range.start..(range.end + 1) {
            let digit_count = i.checked_ilog10().unwrap_or(0) + 1;
            if digit_count % 2 != 0 {
                continue;
            }

            let divider = i64::from(10).pow(digit_count / 2);
            let first_part = i / divider;
            let second_part = i % divider;

            if first_part == second_part {
                result.push(i);
            }
        }

        result
    }

    fn parse_input(&self) -> Vec<Range> {
        let mut result: Vec<Range> = Vec::new();

        let ranges = self.input.split(',').enumerate();

        for (_, rng) in ranges {
            let mut t = rng.split('-').enumerate();
            let start: i64 = t.next().unwrap().1.parse().expect("Failed to parse str");
            let end: i64 = t.next().unwrap().1.parse().expect("Failed to parse str");

            result.push(Range { start, end });
        }

        result
    }
}
