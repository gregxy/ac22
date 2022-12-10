fn main() {
    let mut x : i32 = 1;
    let mut history: Vec<i32> = Vec::new();
    for line in std::io::stdin().lines().map(|l| l.unwrap()) {
        let v: Vec<&str> = line.as_str().split_ascii_whitespace().collect();
        if v[0] == "noop" {
            history.push(x);
        } else {
            history.push(x);
            history.push(x);
            x += v[1].parse::<i32>().unwrap();
        }
    }

    for i in 0..history.len() {
        if i % 40 == 0 {
            println!("");
        }

        let j = (i as i32) % 40;
        if j >= history[i] - 1 && j <= history[i] + 1 {
            print!("{}", '#');
        } else {
            print!("{}", '.');
        }
    }
}
