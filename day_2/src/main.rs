use std::io;
use std::io::BufRead;

struct Policy {
    lb: usize,
    ub: usize,
    target: char,
    password: String
}

fn into_policy(l: &String) -> Policy {
    let v: Vec<&str> = l.split(" ").collect();
    let bounds: Vec<usize> = v[0].split("-")
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    let target: char = v[1].chars().next().unwrap();

    Policy {
        lb: bounds[0],
        ub: bounds[1],
        target: target,
        password: v[2].to_string()
    }
}

fn part_1(l: &String) -> bool {
    let p = into_policy(l);

    let occurrences = p.password.chars()
        .filter(|c| *c == p.target)
        .count();
    
    occurrences >= p.lb && occurrences <= p.ub
}

fn part_2(l: &String) -> bool {
    let p = into_policy(l);

    let a = p.password.chars().nth(p.lb-1).unwrap();
    let b = p.password.chars().nth(p.ub-1).unwrap();

    (a == p.target) ^ (b == p.target)
}

fn main() {
    let v: Vec<String> = io::stdin().lock()
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let n = v.iter().filter(|line| part_1(line)).count();
    println!("{}", n);

    let m = v.iter().filter(|line| part_2(line)).count();
    println!("{}", m);
}
