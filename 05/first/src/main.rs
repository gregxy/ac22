use regex::Regex;

fn main() {
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let mut stack : Vec<Vec<char>> = Vec::new();
    stack.push("DHNQTWVB".chars().collect::<Vec<char>>());
    stack.push("DWB".chars().collect::<Vec<char>>());
    stack.push("TSQWJC".chars().collect::<Vec<char>>());
    stack.push("FJRNZTP".chars().collect::<Vec<char>>());
    stack.push("GPVJMST".chars().collect::<Vec<char>>());
    stack.push("BWFTN".chars().collect::<Vec<char>>());
    stack.push("BLDQFHVN".chars().collect::<Vec<char>>());
    stack.push("HPFR".chars().collect::<Vec<char>>());
    stack.push("ZSMBLNPH".chars().collect::<Vec<char>>());

    for line in std::io::stdin().lines().map(|l| l.unwrap()) {
        let caps_opt = re.captures(line.as_str());
        if caps_opt.is_none() {
            continue;
        }

        let caps = caps_opt.unwrap();
        let count = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let src = caps.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1;
        let dst = caps.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1;

        for _ in 0..count {
            let ch = stack[src].pop().unwrap();
            stack[dst].push(ch);
        }
    }

    println!("{}", stack.iter().map(|x| x[x.len() - 1]).collect::<String>());
}
