use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let mut lines = io::stdin().lock().lines();
    let mut biggest: [u32; 3] = [0; 3];

    let mut tmp: u32 = 0;

    while let Some(line) = lines.next() {
        let last_input = line.unwrap();

        // stop reading
        if last_input.len() == 0 {
            biggest.sort();            
            if tmp > biggest[0] {
                biggest[0] = tmp;
                biggest.rotate_right(1);
            }
            tmp = 0;
            continue;
        }

        let last_input_integer: u32 = last_input.parse().unwrap();
        tmp = tmp + last_input_integer;
    }

    // the lock is released after it goes out of scope
    let sum: u32 = biggest.iter().sum();
    println!("{sum}");
    Ok(())
}