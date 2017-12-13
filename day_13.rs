use std::fs::File;
use std::io::Read;

fn main() {
    let mut f = File::open("day_13_input.txt").expect("File not found");
    let mut _inp = String::new();
    f.read_to_string(&mut _inp).expect("Something went wrong");

    let mut severity: i32 = 0;
    let mut delay = 0;
    let mut poss: bool = false;

    let lines = _inp.lines();
    let mut walls: Vec<(i32, i32)> = Vec::new();

    for line in lines {
        let mut words = line.split_whitespace();
        let depth: i32 = words.next().unwrap().trim_matches(':').parse().unwrap();
        let range: i32 = words.next().unwrap().parse().unwrap();
        walls.push((depth, range));
    }

    while !poss {
        delay += 1;
        poss = true;
        for &(depth, range) in walls.iter() {
            if (depth+delay) % ((range-1)*2) == 0 {
                // Part 1
                // severity += depth*range;
                poss = false;
            }
        }
    }

    println!("{:?}", severity);
    println!("{:?}", delay);

}
