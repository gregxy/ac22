use std::collections::HashSet;

fn main() {
    let mut total: u32 = 0;
    for line in std::io::stdin().lines().map(|l| l.unwrap()) {
        let mut set: HashSet<u8> = HashSet::new();
        let length = line.len();

        for (i, b) in line.as_str().bytes().enumerate() {
            if i < length / 2 {
                set.insert(b);
            } else {
                if set.contains(&b) {
                    if b >= b'a' && b <= b'z' {
                        total += (b - b'a') as u32 + 1;
                    } else {
                        total += (b - b'A') as u32 + 27;
                    }
                    break;
                }
            }
        }
    }

    println!("{}", total);
}
