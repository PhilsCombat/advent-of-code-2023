use std::fs::File;
use std::io::{BufRead, BufReader};

enum SumState {
    SearchFirst,
    SearchSecond,
}
struct Sum {
    sum: u32,
    state: SumState,
    digit_first: u32,
    digit_second: u32,
}

impl Sum {
    fn create() -> Sum {
        return Sum {
            sum: 0,
            digit_first: 0,
            digit_second: 0,
            state: SumState::SearchFirst,
        };
    }
    fn accumulate(&mut self) {
        self.sum += self.digit_first * 10 + self.digit_second;
        self.state = SumState::SearchFirst;
    }

    fn set_first(&mut self, digit: u32) {
        if !matches!(self.state, SumState::SearchFirst) {panic!("tried to set_first whilst state not searching for first digit")}
        self.digit_first = digit;
        self.digit_second = digit;
        self.state = SumState::SearchSecond;
    }

    fn set_second(&mut self, digit: u32) {
        if !matches!(self.state, SumState::SearchSecond) {panic!("tried to set_first whilst state not searching for second digit")}
        self.digit_second = digit;
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut sum = Sum::create();

    let file = File::open(&"./assets/input.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        for c in line.expect("no line").chars() {
            if !c.is_digit(10) { continue; }
            let digit = c.to_digit(10).unwrap();
            match sum.state {
                SumState::SearchFirst => sum.set_first(digit),
                SumState::SearchSecond => sum.set_second(digit),
            }
        }
        sum.accumulate();
    }
    print!("total: {}\n", sum.sum);
    Ok(())
}


