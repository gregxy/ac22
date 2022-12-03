use std::error::Error;
use std::result::Result;

fn main() -> Result<(), Box<dyn Error>> {
    let mut tops: Vec<i32> = vec![0, 0, 0];
    let mut curr : i32 = 0;

    for line in std::io::stdin().lines().map(|l| l.unwrap()) {
        if line.is_empty() {
            for i in 0..3 {
                if curr > tops[i] {
                    (curr, tops[i]) = (tops[i], curr);
                }
            }

            curr = 0;
            continue;
        }
        
        curr += line.parse::<i32>().unwrap();
    }
    
    println!("{}", tops.iter().sum::<i32>());

    Ok(())
}
