use std::io::{self, BufRead};
use std::collections::VecDeque;


fn main() -> io::Result<()> {
    let mut lines = io::stdin().lock().lines();

    let mut vec: Vec<VecDeque<char>> = Vec::new();
    
    let mut tmp: VecDeque<char> = VecDeque::with_capacity(8);
    tmp.push_back('F');
    tmp.push_back('T');
    tmp.push_back('C');
    tmp.push_back('L');
    tmp.push_back('R');
    tmp.push_back('P');
    tmp.push_back('G');
    tmp.push_back('Q');
    vec.push(tmp);

    let mut tmp: VecDeque<char> = VecDeque::with_capacity(8);
    tmp.push_back('N');
    tmp.push_back('Q');
    tmp.push_back('H');
    tmp.push_back('W');
    tmp.push_back('R');
    tmp.push_back('F');
    tmp.push_back('S');
    tmp.push_back('J');
    vec.push(tmp);

    let mut tmp: VecDeque<char> = VecDeque::with_capacity(8);
    tmp.push_back('F');
    tmp.push_back('B');
    tmp.push_back('H');
    tmp.push_back('W');
    tmp.push_back('P');
    tmp.push_back('M');
    tmp.push_back('Q');
    vec.push(tmp);

    let mut tmp: VecDeque<char> = VecDeque::with_capacity(8);
    tmp.push_back('V');
    tmp.push_back('S');
    tmp.push_back('T');
    tmp.push_back('D');
    tmp.push_back('F');
    vec.push(tmp);

    let mut tmp: VecDeque<char> = VecDeque::with_capacity(8);
    tmp.push_back('Q');
    tmp.push_back('L');
    tmp.push_back('D');
    tmp.push_back('W');
    tmp.push_back('V');
    tmp.push_back('F');
    tmp.push_back('Z');
    vec.push(tmp);

    let mut tmp: VecDeque<char> = VecDeque::with_capacity(8);
    tmp.push_back('Z');
    tmp.push_back('C');
    tmp.push_back('L');
    tmp.push_back('S');
    vec.push(tmp);

    let mut tmp: VecDeque<char> = VecDeque::with_capacity(8);
    tmp.push_back('Z');
    tmp.push_back('B');
    tmp.push_back('M');
    tmp.push_back('V');
    tmp.push_back('D');
    tmp.push_back('F');
    vec.push(tmp);

    let mut tmp: VecDeque<char> = VecDeque::with_capacity(8);
    tmp.push_back('T');
    tmp.push_back('J');
    tmp.push_back('B');
    vec.push(tmp);

    let mut tmp: VecDeque<char> = VecDeque::with_capacity(8);
    tmp.push_back('Q');
    tmp.push_back('N');
    tmp.push_back('B');
    tmp.push_back('G');
    tmp.push_back('L');
    tmp.push_back('S');
    tmp.push_back('P');
    tmp.push_back('H');  
    vec.push(tmp);

    while let Some(line) = lines.next() {
        let line = line.unwrap();

        if line.contains("move") {
            let split = line.split(' ');
            let text_splitted = split.collect::<Vec<&str>>();

            let how_many: u32 = text_splitted[1].parse().unwrap();
            let source: usize =  text_splitted[3].parse().unwrap();
            let destination: usize = text_splitted[5].parse().unwrap();

            let mut tmp_stack: VecDeque<char> = VecDeque::with_capacity(8);
            for _i in 0..how_many {
                let element = &vec[source - 1].pop_back().unwrap();
                tmp_stack.push_front(*element);
            }

            for i in tmp_stack {
                &vec[destination - 1].push_back(i);
            }
        }

        // stop reading
        if line.len() == 0 {
            continue;
        }
     }

    for mut i in vec {
        print!("{:?}", &i.pop_back().unwrap())
    }

    Ok(())
}