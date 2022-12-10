use std::collections::HashSet;
use std::io::{ self, BufRead };

#[derive(Hash, Eq, PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn print(&self) {
        println!("({}) ({})", self.x, self.y);
    }

    fn rope_move(&mut self, direction :&str) {
        match direction {
            "L" => self.y -= 1,
            "R" => self.y += 1,
            "U" => self.x += 1,
            "D" => self.x -= 1,
            _ => panic!(),
        }
    }

    fn rope_move_point(&mut self, point :&Point) {
        self.x += point.x;
        self.y += point.y;
    }

    fn should_move(&self, head: &Point) -> Point {
        if self.x >= head.x - 1 && self.x <= head.x + 1 && self.y >= head.y - 1 && self.y <= head.y + 1 {
            return Point {x: 0, y: 0};
        }

        //1
        if self.x == head.x + 1 && self.y == head.y - 2 {
            return Point {x: -1, y: 1};
        }

        //2
        if self.x == head.x && self.y == head.y - 2 {
            return Point {x: 0, y: 1};
        }

        //3
        if self.x == head.x - 1 && self.y == head.y - 2 {
            return Point {x: 1, y: 1};
        }

        //4
        if self.x == head.x - 2 && self.y == head.y - 1 {
            return Point {x: 1, y: 1};
        }

        //5
        if self.x == head.x - 2 && self.y == head.y {
            return Point {x: 1, y: 0};
        }

        //6
        if self.x == head.x - 2 && self.y == head.y + 1 {
            return Point {x: 1, y: -1};
        }

        //7
        if self.x == head.x - 1 && self.y == head.y + 2 {
            return Point {x: 1, y: -1};
        }

        //8
        if self.x == head.x && self.y == head.y + 2 {
            return Point {x: 0, y: -1};
        }

        //9
        if self.x == head.x + 1 && self.y == head.y + 2 {
            return Point {x: -1, y: -1};
        }

        //10
        if self.x == head.x + 2 && self.y == head.y + 1 {
            return Point {x: -1, y: -1};
        }

        //11
        if self.x == head.x + 2 && self.y == head.y {
            return Point {x: -1, y: 0};
        }

        //12
        if self.x == head.x + 2 && self.y == head.y - 1 {
            return Point {x: -1, y: 1};
        }

        if self.x == head.x + 2 && self.y == head.y + 2 {
            return Point {x: -1, y: -1};
        }

        if self.x == head.x + 2 && self.y == head.y - 2 {
            return Point {x: -1, y: 1};
        }

        if self.x == head.x - 2 && self.y == head.y - 2 {
            return Point {x: 1, y: 1};
        }

        if self.x == head.x - 2 && self.y == head.y + 2 {
            return Point {x: 1, y: -1};
        }

        self.print();
        head.print();
        panic!();
    }

    fn tail_move(&mut self, head: Point)  {
        self.rope_move_point(&self.should_move(&head));
    }
}

fn main() -> io::Result<()> {
    let mut visited = HashSet::<Point>::new();
    
    let mut snake = vec![];

    for _ in 0..10 {
        snake.push(Point {x: 0, y: 0});
    }

    let mut lines = io::stdin().lock().lines();
    while let Some(line) = lines.next() {
        let line_split = line.unwrap();
        let splitted = line_split.split(" ").collect::<Vec<&str>>();
        let direction = splitted[0];
        let amount: u32 = splitted[1].parse().unwrap();
        
        visited.insert(Point {x:snake[9].x, y: snake[9].y});
        
        for _ in 0..amount {
            snake[0].rope_move(direction);
            for part in 1..snake.len() {
                let tpm = Point{x: snake[part-1].x, y: snake[part-1].y};
                snake[part].tail_move(tpm);
            }
            visited.insert(Point {x:snake[9].x, y: snake[9].y});
        }
    }

    println!("{}", visited.len());
    Ok(())
}