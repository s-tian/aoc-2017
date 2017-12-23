use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

fn prime(x: i32) -> bool {
    let mut i = 2;
    while i*i <= x {
        if x % i == 0 { 
            return false;
        }
        i += 1;
    }
    return true;
}

fn main() {

    let mut f = File::open("day_23_input.txt").expect("File not found");
    let mut _inp = String::new();
    f.read_to_string(&mut _inp).expect("Something went wrong");
    
    let lines: Vec<&str> = _inp.lines().collect();
    let mut map = HashMap::new();
    let mut ind = 0;
    let mut ans = 0;

    let ascii_iter = (0..8).map(|x| (x + 'a' as u8) as char);
    for c in ascii_iter {
        map.insert(c.to_string(), 0);
    }

    while ind >= 0 && ind < lines.len() as i64 {
        let mut line = lines[ind as usize].split_whitespace();
        let cmd = line.next().unwrap();
        let arg1 = line.next().unwrap();
        let arg1_val: i64; 
        let arg1_opt = arg1.parse();
        if arg1_opt.is_err() {
            arg1_val = *map.get(arg1).unwrap();
        } else {
            arg1_val = arg1_opt.unwrap();
        }

        let arg2 = line.next();
        let mut arg2_val: i64 = 0;
        if !arg2.is_none() {
            let arg2u = arg2.unwrap();
            let arg2_opt = arg2u.parse();
            if arg2_opt.is_err() {
                arg2_val = *map.get(arg2u).unwrap();
            } else {
                arg2_val = arg2_opt.unwrap();
            }
        }

        match cmd {
            // "snd" => last = arg1_val,
            "set" => *map.get_mut(arg1).unwrap() = arg2_val,
            "sub" => *map.get_mut(arg1).unwrap() -= arg2_val,
            "mul" => {
                *map.get_mut(arg1).unwrap() *= arg2_val;
                ans += 1;
            },
            "jnz" => {
                if arg1_val != 0 {
                    ind += arg2_val - 1;
                }
            }
            _ => println!("Error!"),
        }
        ind += 1;
    }

    println!("{:?}", ans);


    // Part 2
    // Find all nonprimes between b and c, stepping by 17
    
    let mut b = 107900;
    let mut h = 0;
    while b <= 124900 {
        if !prime(b) {
            h += 1;
        }
        b += 17;
    }
    
    println!("{:?}", h);
}

