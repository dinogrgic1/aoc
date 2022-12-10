use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let mut lines = io::stdin().lock().lines();
    let mut root = treeline::Tree::new(String::from("/"), vec![]);
    
    let mut curr_node = root;
    let mut prev_parent = String::new();
    while let Some(line) = lines.next() {
        let line = line.unwrap();

        if line.contains("$ cd") {
            let curr_line = line;
            let path = curr_line.split(' ').collect::<Vec<&str>>()[2];
            
            if path == "/" {
                continue;
            } else if path == ".." {
                curr_node = treeline::Tree::root(prev_parent);
            }
            else {
                prev_parent = String::from(path);
                curr_node = treeline::Tree::new(String::from(path), vec![]);
                root.push(curr_node);
            }
        }
        //println!("{:?}", root);
    }
    Ok(())
}