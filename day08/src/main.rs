use std::io::{ self, BufRead };

fn is_visible(trees: &mut Vec<Vec<u32>>, i: usize, j: usize) -> u32 {
    let curr_element = trees[i][j];

    let mut first = 0;
    for j_ in (0..j).rev() {
        first += 1;
        if curr_element <= trees[i][j_] {
            break;
        }
    }

    let mut second = 0;
    for j_ in j + 1..99 {
        second += 1;
        if curr_element <= trees[i][j_] {
            break;
        }
    }

    let mut third = 0;
    for i_ in (0..i).rev() {
        third += 1;
        if curr_element <= trees[i_][j] {
            break;
        }
    }

    let mut fourth = 0;
    for i_ in i + 1..99 {
        fourth += 1;
        if curr_element <= trees[i_][j] {
            break;
        }
    }

    return first * second * third * fourth;
}

fn main() -> io::Result<()> {
    let mut trees = vec![vec![0; 99]; 99];

    let mut lines = io::stdin().lock().lines();

    let mut line_idx = 0;
    while let Some(line) = lines.next() {
        let mut idx = 0;
        for digit in line.unwrap().chars() {
            trees[line_idx][idx] = digit as u32;
            idx += 1;
        }
        line_idx += 1;
    }

    let mut score = 0;
    for i in 1..trees.len() {
        for j in 1..trees[i].len() {
            let tmp = is_visible(&mut trees, i, j);
            if tmp > score {
                score = tmp;
            }
        }
    }

    println!("{:?}", score);

    Ok(())
}