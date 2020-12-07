#![allow(unused)]

use regex::Regex;
use std::io::{self, Read};

fn check_field_1(passport: &str) -> bool {
    ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"].iter()
        .all(|field| passport.contains(field)) 
}

fn check_field_2(passport: &str) -> bool {
    let re = Regex::new(r"([a-z]{3}):(.*?)\s").unwrap();
    re.captures_iter(passport)
        .all(|c|{
            match c {
                c if &c[1] == "byr" => {
                    let year = &c[2].parse::<i32>().unwrap();
                    *year >= 1920 && *year <= 2002
                },
                c if &c[1] == "iyr" => {
                    let year = &c[2].parse::<i32>().unwrap();
                    *year >= 1920 && *year <= 2002
                },
                _ => false
            }
        })
}

fn parse(input: &String) -> usize {
    input.split("\n\n")
        .filter(|passport| check_field_2(passport))
        .count()
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).expect("stdin err");
    
    println!("{}", parse(&buffer));
}
