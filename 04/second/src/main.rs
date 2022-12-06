use regex::Regex;

fn main() {
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    let mut count = 0;

    for line in std::io::stdin().lines().map(|l| l.unwrap()) {
        let caps = re.captures(line.as_str()).unwrap();
        
        let l0 : i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let r0 : i32 = caps.get(2).unwrap().as_str().parse().unwrap();
        let l1 : i32 = caps.get(3).unwrap().as_str().parse().unwrap();
        let r1 : i32 = caps.get(4).unwrap().as_str().parse().unwrap();
       
        if (l0 <= l1 && l1 <= r0) || (l1 <= l0 && l0 <= r1) {
            count += 1;
        }
    }

    println!("{}", count);
}
