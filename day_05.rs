use std::fs::File;
use std::io::Read;

fn main() {
    let mut f = File::open("day_5_input.txt").expect("File not found");
    let mut _inp = String::new();
    f.read_to_string(&mut _inp).expect("Something went wrong");

    let mut nums: Vec<i32> = _inp.lines().map(|s| s.parse().unwrap()).collect();

    let mut i: i32 = 0;
    let mut steps: i32 = 0;
    while i >= 0 && (i as usize) < nums.len() {
        // Part 1:
        // nums[i as usize] += 1;
        // i += nums[i as usize] - 1;
        let change: i32;
        if nums[i as usize] >= 3 {
            change = -1;
        } else {
            change = 1;
        }
        nums[i as usize] += change;
        i += nums[i as usize] - change;
        steps += 1;
    }
    println!("{:?}", steps);

}
