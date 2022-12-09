use std::vec::Vec;

fn main() {
    let mut mat: Vec<Vec<i32>> = Vec::new();
    
    for line in std::io::stdin().lines().map(|l| l.unwrap()) {
        let mut row: Vec<i32> = Vec::new();
        for b in line.as_str().trim().bytes() {
            row.push((b - b'0') as i32);
        }
        mat.push(row);
    }

    let m = mat.len();
    let n = mat[0].len();
    let mut max: i32 = 0;
    for i in 0..m {
        for j in 0..n {
            let mut score : i32 = 1;

            let mut count : i32 = 0;
            for p in (j+1)..n {
                count += 1;
                if mat[i][j] <= mat[i][p] {
                    break;
                }
            }
            score *= count;

            count = 0;
            for p in (i+1)..m {
                count += 1;
                if mat[i][j] <= mat[p][j] {
                    break;
                }
            }
            score *= count;

            count = 0;
            for p in (0..j).rev() {
                count += 1;
                if mat[i][j] <= mat[i][p] {
                    break;
                }
            }
            score *= count;

            count = 0;
            for p in (0..i).rev() {
                count += 1;
                if mat[i][j] <= mat[p][j] {
                    break;
                }
            }
            score *= count;

            if score > max {
                max = score;
            }
        }
    }

    println!("{}", max);
}
