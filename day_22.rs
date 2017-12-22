use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

fn main() {
    let mut f = File::open("day_22_input.txt").expect("File not found");
    let mut _inp = String::new();
    f.read_to_string(&mut _inp).expect("Something went wrong");

    let lines = _inp.lines();

    let mut ans = 0;
    let mut grid = HashMap::new(); 
    let mut r = 0;
    let mut c = 0;
    for line in lines {
        c = 0;
        for ch in line.chars() {
            grid.insert((r, c), ch);
            c += 1;
        }
        r += 1;
    }

    let mut pos = (r/2, c/2);
    let mut dir = (-1, 0);

    for _ in 0..10000000 {
        if !grid.contains_key(&pos) {
            grid.insert(pos, '.');
        }
        let curr = *grid.get(&pos).unwrap();
        match curr {
            '#' => {
                match dir {
                    (-1, 0) => dir = (0, 1),
                    (0, 1) => dir = (1, 0),
                    (1, 0) => dir = (0, -1),
                    (0, -1) => dir = (-1, 0),
                    _ => println!("Error"),
                }
                *grid.get_mut(&pos).unwrap() = 'F';
            },
            '.' => {
                match dir {
                    (-1, 0) => dir = (0, -1),
                    (0, -1) => dir = (1, 0),
                    (1, 0) => dir = (0, 1),
                    (0, 1) => dir = (-1, 0),
                    _ => println!("Error"),
                }
                *grid.get_mut(&pos).unwrap() = 'W';
            },
            'F' => {
                dir = (-dir.0, -dir.1);
                *grid.get_mut(&pos).unwrap() = '.';
            },
            'W' => {
                *grid.get_mut(&pos).unwrap() = '#';
                ans += 1;
            }
            _ => println!("Error!"),
        } 
        pos = (pos.0 + dir.0, pos.1 + dir.1);
    }
    println!("{:?}", ans);
}
