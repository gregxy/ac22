use std::collections::HashMap;
use std::collections::VecDeque;

fn adjust(mat: &Vec<Vec<u8>>, old_high: u8, new_point: (usize, usize),
        cost: usize, visited: &mut HashMap<(usize, usize), usize>,
        pending: &mut VecDeque<(usize, usize)>) {
    let new_high = mat[new_point.0][new_point.1];
    if new_high == b'S' {
        return;
    }

    if new_high != b'E' && new_high > old_high && new_high - old_high > 1 {
        return;
    }

    match visited.get_mut(&new_point) {
        None => {
            visited.insert(new_point.clone(), cost.clone());
            pending.push_back(new_point.clone());
        },
        Some(oc) => {
            if *oc <= cost {
                return;
            }
            *oc = cost;
            pending.push_back(new_point.clone());
        }
    }
}

fn main() {
    let mut mat: Vec<Vec<u8>> = Vec::new();
    let mut visited: HashMap<(usize, usize), usize> = HashMap::new();
    let mut pending: VecDeque<(usize, usize)> = VecDeque::new();

    for line in std::io::stdin().lines().map(|l| l.unwrap()) {
        mat.push(Vec::from(line.as_bytes()));
    }

    let m = mat.len();
    let n = mat[0].len();

    visited.insert((0,0), 0);
    pending.push_back((0,0));

    while !pending.is_empty() {
        let point = pending.pop_front().unwrap();
        let cost = visited.get(&point).unwrap() + 1;
        let old_high = mat[point.0][point.1];
        if old_high == b'E' {
            continue;
        }

        // expand
        if point.0 > 0 {
            let new_point = (point.0 - 1, point.1);
            adjust(&mat, old_high, new_point, cost, 
                &mut visited, &mut pending); 
        }

        if point.0 < m - 1 {
            let new_point = (point.0 + 1, point.1);
            adjust(&mat, old_high, new_point, cost, 
                &mut visited, &mut pending);  
        }

        if point.1 > 0 {
            let new_point = (point.0, point.1 - 1);
            adjust(&mat, old_high, new_point, cost, 
                &mut visited, &mut pending); 
        }

        if point.1 < n - 1 {
            let new_point = (point.0, point.1 + 1);
            adjust(&mat, old_high, new_point, cost, 
                &mut visited, &mut pending);  
        }
    }

    for i in 0..m {
        for j in 0..n {
            if mat[i][j] == b'E' {
                println!("{:?} {}", (i,j), visited.get(&(i, j)).unwrap());
                return;
            }
        }
    }
}

