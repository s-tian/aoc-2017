use std::fs::File;
use std::io::Read;

fn rotate(numbers: &mut Vec<i32>, start: usize, len: usize) {
    let total = numbers.len();
    for i in 0..len/2 {
        numbers.swap((start+i) % total, (start+len-i-1) % total); 
    }
}

fn ff(map: &mut Vec<Vec<u32>>, row: usize, col: usize, color: u32) -> bool {
    if row > 127 || col > 127 || map[row][col] != 1 {
        return false;
    } else {
        map[row][col] = color;
        for (r, c) in vec![(1, 0), (0, 1), (-1, 0), (0, -1)] {
            ff(map, (row as i32 + r) as usize, (col as i32 + c) as usize, color);
        }
        return true;
    }
}

fn main() {
    let mut f = File::open("day_14_input.txt").expect("File not found");
    let mut _inp = String::new();
    f.read_to_string(&mut _inp).expect("Something went wrong");
    _inp = _inp.trim().to_string(); // Remove newline

    let mut used_squares: u32 = 0; 
    let mut region_map = Vec::new();

    for j in 0..128 {
        let hash_inp = _inp.clone() + "-" + &j.to_string();
       
        let mut lens: Vec<usize> = hash_inp.as_bytes().iter().map(|s| *s as usize).collect();
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

        let mut row: Vec<u32> = Vec::new();

        for &num in &dense_hash {
            let mut double_hex_num: u32 = num as u32;
            let mut bin_rep = Vec::new(); 
            for _ in 0..8 {
                if double_hex_num % 2 == 1 {
                   used_squares += 1; 
                }
                bin_rep.push(double_hex_num % 2);
                double_hex_num /= 2;
            }
            for &i in bin_rep.iter().rev() {
                row.push(i);
            }    
        }
        
        region_map.push(row);
    }

    // Part 1
    println!("{:?}", used_squares);
    
    let mut color: u32 = 2; 
    let mut num_regs: u32 = 0;
    for i in 0..128 {
        for j in 0..128 {
            if ff(&mut region_map, i, j, color) {
               color += 1; 
               num_regs += 1;
            }
        }
    }
    // Part 2
    println!("{:?}", num_regs);
}
