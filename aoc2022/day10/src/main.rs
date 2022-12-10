use std::io::{ self, BufRead };

fn check_cycle_add_signal(cycle: i32, value: i32, values: &mut Vec<i32>) {
    if cycle == 20 || cycle == 60 || cycle == 100 || cycle == 140 || cycle == 180 || cycle == 220 {
        values.push(value * cycle);
    }
}

fn draw_pixel(cycle: i32, register: i32) {
    if cycle % 40 == 0 {
        println!();
    }
    
    if register >= cycle % 40 - 1 && register <= cycle % 40 + 1  {
        print!("#");
    } else {
        print!(".");
    }
}

fn main() -> io::Result<()> {
    let mut lines = io::stdin().lock().lines();
    let mut cycle = 1;
    let mut signals = vec![];
    let mut register = 1;

    println!("Task #2 letters: ");
    while let Some(line) = lines.next() {
        let line_split = line.unwrap();
        let splitted = line_split.split(" ").collect::<Vec<&str>>();
             
        let instruction = splitted[0];
        draw_pixel(cycle-1, register);
        if instruction == "addx" {
            let value: i32 = splitted[1].parse().unwrap();
            cycle += 1;
            check_cycle_add_signal(cycle, register, &mut signals);
            draw_pixel(cycle-1, register);
            cycle += 1;
            register += value;
        } else {
            cycle += 1;
        }
        check_cycle_add_signal(cycle, register, &mut signals);
    }

    println!("\n");
    println!("Task #1 solution: {:?}", signals.iter().sum::<i32>());
    Ok(())
}