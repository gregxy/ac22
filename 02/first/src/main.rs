use std::collections::HashMap;

fn main() {
    // A for Rock, B for Paper, and C for Scissors.
    // X for Rock, Y for Paper, and Z for Scissors.
    let m: HashMap<(char, char), i32> = HashMap::from([
        (('A', 'X'), 3),
        (('A', 'Y'), 6),
        (('A', 'Z'), 0),
        (('B', 'X'), 0),
        (('B', 'Y'), 3),
        (('B', 'Z'), 6),
        (('C', 'X'), 6),
        (('C', 'Y'), 0),
        (('C', 'Z'), 3)
    ]);

    let s: HashMap<char, i32> =HashMap::from([
        ('X', 1),
        ('Y', 2),
        ('Z', 3),
    ]);

    let mut total: i32 = 0;

    for line in std::io::stdin().lines().map(|l| l.unwrap()) {
        let cc: Vec<char> = 
            line.as_str()
                .split_whitespace()
                .map(|s| s.chars().next().unwrap())
                .collect();

        total += m.get(&(cc[0], cc[1])).unwrap() + s.get(&cc[1]).unwrap();
    }

    println!("{}", total);
}
