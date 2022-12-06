use std::collections::VecDeque;

fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();

    let mut que: VecDeque<char> = VecDeque::new();

    for (index, c) in line.as_str().chars().enumerate() {
        let mut rcount: usize = 0;

        for i in 0..que.len() {
            if que[i] == c {
                rcount = i + 1;
                break;
            }
        }

        for _ in 0..rcount {
            que.pop_front();
        }

        que.push_back(c);

        if que.len() == 4 {
            println!("{}", index + 1);
            break;
        }
    }
}
