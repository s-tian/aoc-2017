use std::fs::File;
use std::io::Read;

fn main() {
    let mut f = File::open("day_15_input.txt").expect("File not found");
    let mut _inp = String::new();
    f.read_to_string(&mut _inp).expect("Something went wrong");
    let mut _it = _inp.lines();
    let mut r1: u64 = _it.nth(0).unwrap().split_whitespace().nth(4).unwrap().parse().unwrap();
    let mut r2: u64 = _it.nth(0).unwrap().split_whitespace().nth(4).unwrap().parse().unwrap();
    let mut ans: u32 = 0;
    
    for _i in 0..5000000 {

        r1 = (r1 * 16807) % 2147483647;
        while !(r1 % 4 == 0) {
            r1 = (r1 * 16807) % 2147483647;
        }

        r2 = (r2 * 48271) % 2147483647;
        while !(r2 % 8 == 0) {
            r2 = (r2 * 48271) % 2147483647;
        }

        if r1 & 0xffff == r2 & 0xffff {
            ans += 1; 
        }
    }

    println!("{:?}", ans);
}
