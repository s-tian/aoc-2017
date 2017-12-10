use std::fs::File;
use std::io::Read;

fn rotate(numbers: &mut Vec<i32>, start: usize, len: usize) {
    let total = numbers.len();
    for i in 0..len/2 {
        numbers.swap((start+i) % total, (start+len-i-1) % total); 
    }
}

fn main () {
    let mut f = File::open("day_10_input.txt").expect("File not found");
    let mut _inp = String::new();
    f.read_to_string(&mut _inp).expect("Something went wrong");
    // Part 1: 
    // let lens: Vec<usize> = _inp.trim().split(",").map(|s| s.parse().unwrap()).collect();
    let mut lens: Vec<usize> = _inp.trim().as_bytes().iter().map(|s| *s as usize).collect();
    lens.append(&mut vec![17, 31, 73, 47, 23]);

    let mut skip: usize = 0; 
    let mut nums: Vec<i32> = (0..256).collect();
    let mut start: usize = 0;
    
    for _ in 0..64 {
        for &len in &lens {
            rotate(&mut nums, start, len);
            start += (skip + len) % nums.len();
            skip += 1;
        } 
    }
    let mut dense_hash: Vec<i32> = Vec::new();

    for i in 0..16 {
        dense_hash.push(nums[i*16..(i+1)*16].iter().fold(0, |total, x| total ^ x));
    }

    let final_string: String = dense_hash.iter().map(|x| format!("{:x}", x)).collect();
   
    println!("{:?}", nums[0]*nums[1]); // Part 1
    println!("{:?}", final_string); // Part 2

}
