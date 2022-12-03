use std::collections::HashMap;

fn main() {
    // A for Rock, B for Paper, and C for Scissors.
    // X for loose, Y draw, and Z for win
    let m: HashMap<(char, char), char> = HashMap::from([
        (('A', 'X'), 'C'),
        (('A', 'Y'), 'A'),
        (('A', 'Z'), 'B'),
        (('B', 'X'), 'A'),
        (('B', 'Y'), 'B'),
        (('B', 'Z'), 'C'),
        (('C', 'X'), 'B'),
        (('C', 'Y'), 'C'),
        (('C', 'Z'), 'A')
    ]);

    let n: HashMap<char, i32> =HashMap::from([
        ('A', 1),
        ('B', 2),
        ('C', 3),
    ]);

    let s: HashMap<char, i32> =HashMap::from([
        ('X', 0),
        ('Y', 3),
        ('Z', 6),
    ]);

    let mut total: i32 = 0;

    for line in std::io::stdin().lines().map(|l| l.unwrap()) {
        let cc: Vec<char> = 
            line.as_str()
                .split_whitespace()
                .map(|s| s.chars().next().unwrap())
                .collect();

        total += n.get(m.get(&(cc[0], cc[1])).unwrap()).unwrap();
        total += s.get(&cc[1]).unwrap();
    }

    println!("{}", total);
}
