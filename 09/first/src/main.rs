use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let lut : HashMap<String, (i32, i32)> = HashMap::from([
        (String::from("U"), (0, 1)),
        (String::from("D"), (0, -1)),
        (String::from("L"), (-1, 0)),
        (String::from("R"), (1, 0))]);
    
    let mut hx: i32 = 0;
    let mut hy: i32 = 0;
    let mut tx: i32 = 0;
    let mut ty: i32 = 0;
    
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((0,0));
    for line in std::io::stdin().lines().map(|l| l.unwrap()) {
        let v: Vec<&str> = line.as_str().split_ascii_whitespace().collect();
        let &(dx, dy) = lut.get(v[0]).unwrap();
        let steps : i32 = v[1].parse().unwrap();
        for _ in 0..steps {
            hx += dx;
            hy += dy;
            
            if ty == hy && (tx - hx == 2) {
                tx -= 1;
            } else if ty == hy && (hx - tx == 2) {
                tx += 1;
            } else if tx == hx && (ty - hy == 2) {
                ty -= 1;
            } else if tx == hx && (hy - ty == 2) {
                ty += 1;
            } else if (hy == ty + 1) && (hx - tx == 2) {
                ty += 1;
                tx += 1;
            } else if (hy == ty - 1) && (hx - tx == 2) {
                ty -= 1;
                tx += 1;
            } else if (hy == ty + 1) && (tx - hx == 2) {
                ty += 1;
                tx -= 1;
            } else if (hy == ty - 1) && (tx - hx == 2) {
                ty -= 1;
                tx -= 1;
            } else if (hx == tx + 1) && (hy - ty == 2) {
                tx += 1;
                ty += 1;
            } else if (hx == tx + 1) && (ty - hy == 2) {
                tx += 1;
                ty -= 1;
            } else if (tx == hx + 1) && (hy - ty == 2) {
                tx -= 1;
                ty += 1;
            } else if (tx == hx + 1) && (ty - hy == 2) {
                tx -= 1;
                ty -= 1;
            }
            
            visited.insert((tx, ty));
        }
    }
    
    println!("{}", visited.len());
}
