use num::{FromPrimitive, Zero};
use once_cell::sync::Lazy;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::{self, Write};
use std::ops::Rem;

fn main() {
    println!("Please enter a list of integers, or 'quit' to quit");
    loop {
        let input = read_line();
        if input == "quit" {
            break;
        }
        let values = match parse_integers(&input) {
            Ok(vs) => vs,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        println!("{:?}", values);

        let mean = calculate_mean(&values);
        println!("mean = {}", mean);

        let median = calculate_median(&values);
        println!("median = {}", median);

        let mode = calculate_mode(&values);
        if mode.len() == 1 {
            println!("mode = {}", mode[0])
        } else {
            println!("mode = {:?}", mode);
        }
    }
}

fn calculate_mean(values: &[i32]) -> f64 {
    let sum = values.iter().sum::<i32>() as f64;
    sum / values.len() as f64
}

#[test]
fn test_mean() {
    assert!(calculate_mean(&[1]) - 1. < std::f64::EPSILON);
    assert!(calculate_mean(&[1, 2]) - 1.5 < std::f64::EPSILON);
}

fn calculate_median(values: &[i32]) -> f64 {
    let n = values.len() as i32;
    let idx = (n / 2) as usize;
    if n.is_odd() {
        values[idx] as f64
    } else {
        let sum = (values[idx - 1] + values[idx]) as f64;
        sum / 2.
    }
}

#[test]
fn test_median() {
    assert!(calculate_median(&[3, 1, 2]) - 2. < std::f64::EPSILON);
    assert!(calculate_median(&[3, 1, 2, 4]) - 2.5 < std::f64::EPSILON);
}

fn calculate_mode(values: &[i32]) -> Vec<i32> {
    let mut counts = HashMap::new();
    for v in values {
        *(counts.entry(*v).or_insert(0)) += 1;
    }
    let mut mode = Vec::new();
    let mut max_count = 0;
    for (v, count) in counts {
        match count.cmp(&max_count) {
            Ordering::Less => (),
            Ordering::Equal => mode.push(v),
            Ordering::Greater => {
                max_count = count;
                mode.clear();
                mode.push(v);
            }
        }
    }
    mode.sort();
    mode
}

#[test]
fn test_mode() {
    assert_eq!(calculate_mode(&[1]), [1]);
    assert_eq!(calculate_mode(&[2, 1, 3]), [1, 2, 3]);
    assert_eq!(calculate_mode(&[1, 3, 2, 3, 4, 3, 2, 3, 3]), [3]);
}

trait ParityCmp {
    fn is_even(&self) -> bool;
    fn is_odd(&self) -> bool;
}

impl<T> ParityCmp for T
where
    T: Rem<Output = T> + PartialEq + FromPrimitive + Copy + Zero,
{
    fn is_even(&self) -> bool {
        let two = T::from_i32(2).unwrap();
        (*self) % two == T::zero()
    }

    fn is_odd(&self) -> bool {
        !self.is_even()
    }
}

#[test]
fn test_parity_cmp() {
    assert!(!0.is_odd());
    assert!(13.is_odd());
    assert!(!12.is_odd());
    assert!((-37).is_odd());
    assert!(!(-38).is_odd());

    assert!(0.is_even());
    assert!(!13.is_even());
    assert!(12.is_even());
    assert!(!(-37).is_even());
    assert!((-38).is_even());
}

macro_rules! lazy {
    ($e:expr) => {
        Lazy::new(|| $e)
    };
}

fn parse_integers(input: &str) -> Result<Vec<i32>, String> {
    const RE_STR: &str = r"[[:space:]|,|;]+";
    static RE: Lazy<Regex> = lazy!(Regex::new(RE_STR).unwrap());
    let mut values: Vec<_> = RE.split(input).map(parse_int).collect::<Result<_, _>>()?;
    if values.is_empty() {
        Err(String::from("Please enter a valid list of integers"))
    } else {
        values.sort();
        Ok(values)
    }
}

fn parse_int(input: &str) -> Result<i32, String> {
    input.parse::<i32>().map_err(|_| {
        format!(
            "'{}' is not a valid integer, please enter a valid list of integers",
            input
        )
    })
}

fn read_line() -> String {
    print!("> ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    String::from(input.trim())
}
