use std::collections::HashSet;

fn main() {
    let mut vs: Vec<HashSet<u8>> = Vec::new();
    let mut total: u32 = 0;
    for line in std::io::stdin().lines().map(|l| l.unwrap()) {
        vs.push(HashSet::from(line.bytes().collect::<HashSet<u8>>()));
        if vs.len() != 3 {
            continue;
        }
        let st: HashSet<u8> = vs[0].intersection(&vs[1]).map(|x| *x).collect();
        for &b in st.intersection(&vs[2]) {
            if b >= b'a' && b <= b'z' {
                total += (b - b'a') as u32 + 1;
            } else {
                total += (b - b'A') as u32 + 27;
            }
            break;
        }
        vs.clear();
    }

    println!("{}", total);
}
