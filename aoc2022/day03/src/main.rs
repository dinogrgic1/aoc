use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let mut lines = io::stdin().lock().lines();

    while let Some(line) = lines.next() {
        let vec = split.collect::<Vec<&str>>();

        let item = line.unwrap().to_string();
        let first = item.substring(0, item.len() / item);
        let second = item.substring(item.len() / 2, item.len());

        // stop reading
        if item.len() == 0 {
            continue;
        }

        let num = item.len(); 
        println!("{num}");

     }
    Ok(())
}