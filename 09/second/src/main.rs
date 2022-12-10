use std::collections::HashMap;
use std::collections::HashSet;

fn sim(hx: i32, hy: i32, tx: i32, ty: i32) -> (i32, i32) {
    let mut tx = tx;
    let mut ty = ty;
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
    } else if (hx - tx == 2) && (hy - ty == 2) {
        tx += 1;
        ty += 1;
    } else if (hx - tx == 2) && (ty - hy == 2) {
        tx += 1;
        ty -= 1;
    } else if (tx - hx == 2) && (hy - ty == 2) {
        tx -= 1;
        ty += 1;
    } else if (tx - hx == 2) && (ty - hy == 2) {
        tx -= 1;
        ty -= 1;
    }
    
    return (tx, ty);
}
  
fn main() {
    let lut : HashMap<String, (i32, i32)> = HashMap::from([
        (String::from("U"), (0, 1)),
        (String::from("D"), (0, -1)),
        (String::from("L"), (-1, 0)),
        (String::from("R"), (1, 0))]);

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((0,0));
    let mut knobs : [(i32, i32); 10] = [(0, 0); 10];
    
    for line in std::io::stdin().lines().map(|l| l.unwrap()) {
        let v: Vec<&str> = line.as_str().split_ascii_whitespace().collect();
        let &(dx, dy) = lut.get(v[0]).unwrap();
        let steps : i32 = v[1].parse().unwrap();
        for _ in 0..steps {
            knobs[0].0 += dx;
            knobs[0].1 += dy;
            
            for i in 1..10 {
                let (lx, ly) = sim(knobs[i-1].0, knobs[i-1].1, knobs[i].0, knobs[i].1);
                knobs[i] = (lx, ly); 
            }
            visited.insert(knobs[9]);
        }
    }
    
    println!("{}", visited.len());
}
