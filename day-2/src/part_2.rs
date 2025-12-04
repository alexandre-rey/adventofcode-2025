struct Range {
    start: i64,
    end: i64,
}

pub struct Part2 {
    input: String,
}

impl Part2 {
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

        let solution: i64 = result.iter().sum();
        println!("Part 2 final solution: {}", solution);
    }

    fn is_repeated_substring(&self, n: i64) -> bool {
        let s = n.to_string();
        let len = s.len();

        // Try each possible pattern length
        for pat_len in 1..=(len / 2) {
            if len % pat_len != 0 {
                continue;
            }

            let repeats = len / pat_len;
            let pat = &s[..pat_len];

            // Build the candidate string
            let candidate = pat.repeat(repeats);

            if candidate == s {
                return true;
            }
        }
        false
    }

    fn split_range(&self, range: Range) -> Vec<i64> {
        let mut result = Vec::new();
        for i in range.start..(range.end + 1) {
            if self.is_repeated_substring(i) {
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
