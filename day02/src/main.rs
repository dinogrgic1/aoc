use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let mut lines = io::stdin().lock().lines();
    let mut score: u32 = 0;

    while let Some(line) = lines.next() {
        let rps = line.unwrap();

        // stop reading
        if rps.len() == 0 {
            continue;
        }

        match rps.as_ref() {
            "A X" => score += 3,
            "A Y" => score += 4,
            "A Z" => score += 8,
            "B X" => score += 1,
            "B Y" => score += 5,
            "B Z" => score += 9,
            "C X" => score += 2,
            "C Y" => score += 6,
            "C Z" => score += 7,
            _=> println!("Only one  digit allowed"),
        }
     }

    println!("{score}");
    Ok(())
}