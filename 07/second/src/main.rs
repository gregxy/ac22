use std::collections::HashMap;
use std::collections::HashSet;

struct Node {
    parent: usize,
    size: i64,
    children: HashMap<String, usize>,
    files: HashSet<String>
}

impl Node {
    fn new(parent: usize) -> Self {
        Self {
            parent,
            size: 0,
            children: HashMap::new(),
            files: HashSet::new(),
        }
    }
}

fn main() {
    let mut pool: Vec<Node> = Vec::new();
    pool.push(Node::new(0));
    let mut curr: usize = 0;

    for line in std::io::stdin().lines().map(|l| l.unwrap()) {
        let v: Vec<&str> = line.as_str().trim().split(' ').collect();
        if v[0] == "$" {
            if v[1] == "cd" {
                if v[2] == ".." {
                    curr = pool[curr].parent;
                } else {
                    match pool[curr].children.get(v[2]) {
                        None => {
                            pool.push(Node::new(curr));
                            let cc = pool.len() - 1;

                            pool[curr].children.insert(
                                String::from(v[2]),
                                cc);
                            curr = cc;
                        }
                        Some(n) => curr = *n,
                    }
                }
            }
            continue;
        }

        if v[0] == "dir" {
            continue;
        }

        let f = String::from(v[1]);
        if pool[curr].files.contains(&f) {
            continue;
        }
        pool[curr].files.insert(f);
        let s = v[0].parse::<i64>().unwrap();
        let mut p = curr;
        while p != 0 {
            pool[p].size += s;
            p = pool[p].parent;
        }
    }

    let req = 30000000 - (70000000 - pool[1].size);
    let mut ss = pool[1].size;

    for i in 2..pool.len() {
        if pool[i].size >= req && pool[i].size < ss {
            ss = pool[i].size;
        }
    }

    println!("{}", ss);
}
