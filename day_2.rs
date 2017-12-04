use std::fs::File;
use std::io::Read;

fn main() {
    let mut f = File::open("day_2_input.txt").expect("File not found");
    let mut _inp = String::new();
    f.read_to_string(&mut _inp).expect("Something went wrong");

    let lines = _inp.lines();
    let mut total: i32 = 0;
    for line in lines {
        let nums: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        // Part 1:
        // total = total + nums.iter().max().unwrap() - nums.iter().min().unwrap();
        // Unfortunately, using ranges to emulate a C style loop
        for i in 0..nums.len() {
            for j in i+1..nums.len() {
                if nums[i] % nums[j] == 0 {
                    total = total + nums[i]/nums[j];
                }
                if nums[j] % nums[i] == 0 {
                    total = total + nums[j]/nums[i];
                }
            }
        }
    }
    println!("{:?}", total)
}
