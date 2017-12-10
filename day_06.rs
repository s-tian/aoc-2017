use std::fs::File;
use std::io::Read;

fn duplicate(seen: &Vec<Vec<i32>>, n: &Vec<i32>) -> i32 {
    let mut checked: usize = 0;
    for arr in seen {
        checked += 1;
        if arr == n {
            return (seen.len()-checked+1) as i32;
        }
    }
    return 0;
}

fn rearrange(nums: &Vec<i32>) -> Vec<i32> {
    let mut highest: i32 = 0;
    let mut highest_ind: usize = 0;
    for (i, n) in nums.iter().enumerate() {
        if *n > highest {
            highest = *n;
            highest_ind = i;
        }
    }

    let mut new: Vec<i32> = nums.iter().cloned().collect();
    new[highest_ind] = 0; 
    let mut c_ind: usize = (highest_ind+1) % new.len();
    while highest > 0 {
        new[c_ind] += 1;
        highest -= 1;
        c_ind = (c_ind + 1) % new.len();
    }
    return new; 
}

fn main() {
    let mut f = File::open("day_6_input.txt").expect("File not found");
    let mut _inp = String::new();
    f.read_to_string(&mut _inp).expect("Something went wrong");

    let mut nums: Vec<i32> = _inp.split_whitespace().map(|s| s.parse().unwrap()).collect();
    
    let mut i: i32 = 0;
    let mut seen: Vec<Vec<i32>> = Vec::new();

    while duplicate(&seen, &nums) == 0 {
        seen.push(nums.iter().cloned().collect()); 
        nums = rearrange(&nums);
        i += 1;
    }
    println!("{:?}", i);
    println!("{:?}", duplicate(&seen, &nums));


}
