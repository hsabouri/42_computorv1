use std::env;
use std::f64::EPSILON;

extern crate regex;
use self::regex::Regex;

use maths;

const VERIFY_REGEX: &str = r"^(\s*[-+]?\s*\d+(\.\d+)?\s*\*\s*[xX]\s*\^\s*[-+]?\s*\d+(\.\d+)?\s*)+=(\s*[-+]?\s*\d+(\.\d+)?\s*\*\s*[xX]\s*\^\s*[-+]?\s*\d+(\.\d+)?\s*)+$";

fn verify(arg: &String) -> bool {
    let reg = Regex::new(VERIFY_REGEX).unwrap();
    reg.is_match(arg)
}

pub fn parse(arg: &String) -> (Option<(maths::Polynomial, maths::Polynomial)>, Option<String>) {
    /*
    let mut equation: Option<(maths::Polynomial, maths::Polynomial)> = None;
    let mut error: Option<String> = None;
    */

    let mut res: (Option<(maths::Polynomial, maths::Polynomial)>, Option<String>) = (None, None);

    if verify(arg) {
        res = (None, None);
    } else {
        res = (None, Some(String::from("Can't parse Equation")));
    }

    res
}

pub fn get_from_args() -> Option<String> {
    let mut equation: Option<String> = None;
    for arg in env::args() {
        println!("ARG => {}", &arg);
        match arg.find("/").is_some() {
            true => {
                println!("Matched path")
            },
            false => {
                equation = Some(arg.clone());
                break;
            }
        }
    }

    equation
}
