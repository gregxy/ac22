use std::collections::HashSet;

fn main() {
    let mut x : i32 = 1;
    let mut cycle: i32 = 1;
    let mut strength : i32 = 0; 

    let points: HashSet<i32> = HashSet::from([20, 60, 100, 140, 180, 220]);

    for line in std::io::stdin().lines().map(|l| l.unwrap()) {
        let v: Vec<&str> = line.as_str().split_ascii_whitespace().collect();

        if points.contains(&cycle) {
            strength += cycle * x;
        } else if points.contains(&(cycle + 1)) && v[0] == "addx" {
            strength += (cycle + 1) * x; 
        }


        if v[0] == "noop" {
            cycle += 1;
        } else {
            x += v[1].parse::<i32>().unwrap();
            cycle += 2;
        }
    }

    println!("{}", strength);
}
