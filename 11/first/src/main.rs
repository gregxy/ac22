struct Monkey {
    items : Vec<u64>,
    op: fn(u64) -> u64,
    div: u64,
    pos: usize,
    neg: usize,
    count: usize,
}

fn main() {
    let mut monkeys : [Monkey; 8] = [
        Monkey {
            items: vec![80],
            op: |x| x * 5,
            div: 2,
            pos: 4,
            neg: 3,
            count: 0,
        },
        Monkey {
            items: vec![75, 83, 74],
            op: |x| x + 7,
            div: 7,
            pos: 5,
            neg: 6,
            count: 0,
        },
        Monkey {
            items: vec![86, 67, 61, 96, 52, 63, 73],
            op: |x| x + 5,
            div: 3,
            pos: 7,
            neg: 0,
            count: 0,
        },
        Monkey {
            items: vec![85, 83, 55, 85, 57, 70, 85, 52],
            op: |x| x + 8,
            div: 17,
            pos: 1,
            neg: 5,
            count: 0,
        },
        Monkey {
            items: vec![67, 75, 91, 72, 89],
            op: |x| x + 4,
            div: 11,
            pos: 3,
            neg: 1,
            count: 0,
        },
        Monkey {
            items: vec![66, 64, 68, 92, 68, 77],
            op: |x| x * 2,
            div: 19,
            pos: 6,
            neg: 2,
            count: 0,
        },
        Monkey {
            items: vec![97, 94, 79, 88],
            op: |x| x * x,
            div: 5,
            pos: 2,
            neg: 7,
            count: 0,
        },
        Monkey {
            items: vec![77, 85],
            op: |x| x + 6,
            div: 13,
            pos: 4,
            neg: 0,
            count: 0,
        },
    ];

    for _ in 0..20 {
        for i in 0..8 {
            monkeys[i].count += monkeys[i].items.len();

            for j in 0..monkeys[i].items.len() {
                let f = monkeys[i].op;
                let v = f(monkeys[i].items[j]) / 3;
                let t = if v % monkeys[i].div == 0 {
                    monkeys[i].pos
                } else {
                    monkeys[i].neg
                };
                monkeys[t].items.push(v);
            }
            monkeys[i].items.clear();
        }
    }
    let mut vv = monkeys.iter().map(|x| x.count).collect::<Vec<usize>>();
    vv.sort();

    println!("{}", vv[7] * vv[6]);
}
