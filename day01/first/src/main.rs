use std::error::Error;
use std::result::Result;

fn main() -> Result<(), Box<dyn Error>> {
    let mut max: i32 = 0;
    let mut curr: i32 = 0;

    for line in std::io::stdin().lines().map(|l| l.unwrap()) {
        if line.is_empty() {
            max = std::cmp::max(max, curr);
            curr = 0;
            continue;
        }

        curr += line.parse::<i32>().unwrap();
    }
    
    println!("{}", max);

    Ok(())
}
