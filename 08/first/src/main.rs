use std::collections::HashSet;
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
    
    let mut visible: HashSet<(usize, usize)> = HashSet::new();
    let mut last: i32;
    for i in 0..mat.len() {
        last = -1;
        for j in 0..mat[0].len() {
            if mat[i][j] > last {
                visible.insert((i,j));
                last = mat[i][j];
            }
        }
        
        last = -1;
        for j in (0..mat[0].len()).rev() {
            if mat[i][j] > last {
                visible.insert((i,j));
                last = mat[i][j];
            }
        }
    }
    
    for j in 0..mat[0].len() {
        last = -1;
        for i in 0..mat.len() {
            if mat[i][j] > last {
                visible.insert((i,j));
                last = mat[i][j];
            }
        }
        
        last = -1;
        for i in (0..mat.len()).rev() {
            if mat[i][j] > last {
                visible.insert((i,j));
                last = mat[i][j];
            }
        }
    }
    
    println!("{}", visible.len());
}
