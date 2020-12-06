use std::io;
use std::io::BufRead;

type Grid = Vec<Vec<char>>;

fn visit(g: &Grid, dx: usize, dy: usize) -> i32 {
    let mut x = 0;
    let mut y = 0;
    let mut n = 0;

    let w = g[0].len();
    let h = g.len();

    while x < h {
        if g[x][y % w] == '#' {
            n += 1;
        }

        x += dx;
        y += dy;
    }

    n
}

fn main() {
    let mut g: Grid = Vec::new();
    for line in io::stdin().lock().lines() {
        g.push(line.unwrap().chars().collect());
    }

    println!("{}", visit(&g, 1, 3));
    println!("{}",
        visit(&g, 1, 1) *
        visit(&g, 1, 3) *
        visit(&g, 1, 5) *
        visit(&g, 1, 7) *
        visit(&g, 2, 1)
    );
}
