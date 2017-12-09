use std::fs::File;
use std::io::Read;

fn main() {
    let mut f = File::open("day_9_input.txt").expect("File not found");
    let mut _inp = String::new();
    f.read_to_string(&mut _inp).expect("Something went wrong");
    
    let mut score: i32 = 0;

    let mut garbage: bool = false;
    let mut cancel: bool = false;
    let mut depth: i32 = 0;
    let mut garbage_count: i32 = 0;

    for char in _inp.chars() {
        if cancel {
            cancel = false;
            continue;
        } else if garbage {
            match char {
                '!' => cancel = true,
                '>' => garbage = false,
                _ => garbage_count += 1,
            }
        } else {
            match char {
                '{' => {
                    depth += 1;
                    score += depth;
                },
                '}' => depth -= 1, 
                '<' => garbage = true,
                _ => continue,
            }
        }

    }
    println!("{:?}", score); // Part 1
    println!("{:?}", garbage_count); // Part 2

}
