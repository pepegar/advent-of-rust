use std::io::{self, Read};

struct Result(i32, i32, i32);

impl Result {
    fn new(a: i32, b: i32, c: i32) -> Result {
        Result(a, b, c)
    }

    fn multiply(&self) -> i32 {
        self.0 * self.1 * self.2
    }
}

fn sum_2020(a: &i32, b: &i32, c: &i32) -> bool {
    a + b + c == 2020
}

fn find_pair(slice: &[i32]) -> Option<Result> {
    for &a in slice {
        for &b in slice {
            for &c in slice {
                if sum_2020(&a, &b, &c) {
                    return Some(Result::new(a, b, c));
                }
            }
        }
    }
    None
}

fn main() {
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut buffer);

    let nums: Vec<i32> = buffer
        .lines()
        .map(|str| str.parse::<i32>().unwrap())
        .collect();

    let value = find_pair(&nums).map(|result| result.multiply());

    match value {
        Some(value) => println!("the result is {}", value),
        None => println!("nOPE"),
    }
}
