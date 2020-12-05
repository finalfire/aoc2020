use std::io;
use std::io::BufRead;

fn part_2(v: &mut Vec<i32>) -> Option<i32> {
    for (i, a) in v.iter().enumerate() {
        let mut j: usize = i + 1;
        let mut k: usize = v.len() - 1;

        while j < k {
            let s = a + v[j] + v[k];
            match s {
                s if s > 2020 => k -= 1,
                s if s < 2020 => j += 1,
                _ => return Some(a * v[j] * v[k])
            }
        } 
    }
    None
}

fn part_1(v: &mut Vec<i32>) -> Option<i32> {
    let mut i: usize = 0;
    let mut j: usize = v.len() - 1;

    while i < j {
        let s = v[i] + v[j];
        match s {
            s if s > 2020 => j -= 1,
            s if s < 2020 => i += 1,
            _ => return Some(v[i] * v[j])
        }
    }
    None
}

fn main() {
    let mut v: Vec<i32> = io::stdin().lock().lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect();
    v.sort();

    println!("{}", part_1(&mut v).unwrap());
    println!("{}", part_2(&mut v).unwrap());
}
