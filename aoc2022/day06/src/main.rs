use std::io::{self, BufRead};

fn unique(s: &str) -> Option<(usize, usize, char)> {
    s.chars().enumerate().find_map(|(i, c)| {
        s.chars()
            .enumerate()
            .skip(i + 1)
            .find(|(_, other)| c == *other)
            .map(|(j, _)| (i, j, c))
    })
}

fn main() -> io::Result<()> {
    let mut lines = io::stdin().lock().lines();
    while let Some(line) = lines.next() {
        let line = line.unwrap();
        for _i in 14..line.len()-14 {
            let signal = &line[_i-14.._i];
            let is_unique = match unique(signal) {
                None => true,
                Some((_i, _j, _c)) => false,
            };

            if is_unique == true {
                println!("{:?}", _i);
                break;
            }
        }
    }
    Ok(())
}