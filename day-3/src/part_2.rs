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
        let mut max_per_line: Vec<u32> = Vec::new();

        for line in self.input.lines() {
            let mut cm = 0;
            for i in 0..line.len() {
                let ci = line.get(i..(i + 1)).unwrap();
                for j in (i + 1)..line.len() {
                    let cj = line.get(j..(j + 1)).unwrap();
                    let mut tmp = ci.to_string();
                    tmp.push_str(cj);
                    let tmp: u32 = tmp.parse().unwrap();
                    if tmp > cm {
                        cm = tmp;
                    }
                }
            }
            println!("line cm: {cm}");
            max_per_line.push(cm);
        }
        let sum: u32 = max_per_line.iter().sum();
        println!("Max joltage: {}", sum);
    }
}
