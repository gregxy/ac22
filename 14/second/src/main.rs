use std::collections::HashSet;
use regex::Regex;

fn main() {
    let tuple_re = Regex::new(r"(\d+),(\d+)").unwrap();
    let mut cave: HashSet<(i32, i32)> = HashSet::new();

    for line in std::io::stdin().lines().map(|l| l.unwrap()) {
        let mut last_x: i32 = -1;
        let mut last_y: i32 = -1;

        for cap in tuple_re.captures_iter(line.as_str()) {
            let x = cap[1].parse::<i32>().unwrap();
            let y = cap[2].parse::<i32>().unwrap();

            if last_x < 0 {
                last_x = x;
                last_y = y;
                continue;
            }

            if x == last_x {
                let min_y = std::cmp::min(last_y, y);
                let max_y = std::cmp::max(last_y, y);
                for yy in min_y..=max_y {
                    cave.insert((x, yy));
                }
            } else {
                let min_x = std::cmp::min(last_x, x);
                let max_x = std::cmp::max(last_x, x);
                for xx in min_x..=max_x {
                    cave.insert((xx, y));
                }
            }

            last_x = x;
            last_y = y;
        }
    }

    let deepest = cave.iter().map(|x| x.1).max().unwrap() + 2;
    let mut count: i32 = 0;

    loop {
        let mut x: i32 = 500;
        let mut y: i32 = 0;
        
        loop {
            if y + 1 == deepest {
                cave.insert((x, y));
                count += 1;
                break;
            }

            if !cave.contains(&(x, y + 1)) {
                y += 1;
                continue;
            }

            if !cave.contains(&(x - 1, y + 1)) {
                y += 1;
                x -= 1;
                continue;
            }

            if !cave.contains(&(x + 1, y + 1)) {
                y += 1;
                x += 1;
                continue;
            }
            
            cave.insert((x, y));
            count += 1;
            break;
        }

        if y == 0 {
            break;
        }
    }

    println!("{}", count);
}

