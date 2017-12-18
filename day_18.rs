use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use std::collections::VecDeque;

// This solution is quite janky, sorry in advance :( 

fn run_prog(lines: &Vec<&str>, map: &mut HashMap<String, i64>, ind: &mut i64, def_val: i64, runningc: &mut bool, runningo: &mut bool, rcvc: &mut VecDeque<i64>, rcvo: &mut VecDeque<i64>, pg1sends: &mut u32) {
    while ind >= &mut 0i64 && *ind < lines.len() as i64 {
        let mut line = lines[*ind as usize].split_whitespace();
        let cmd = line.next().unwrap();
        let arg1 = line.next().unwrap();
        let mut arg1_val: i64 = 0; 
        let arg1_opt = arg1.parse();
        if arg1_opt.is_err() {
            if map.contains_key(arg1) {
                arg1_val = *map.get(arg1).unwrap();
            } else {
                map.insert(arg1.to_string().clone(), def_val);
            }
        } else {
            arg1_val = arg1_opt.unwrap();
        }

        let arg2 = line.next();
        let mut arg2_val: i64 = 0;
        if !arg2.is_none() {
            let arg2u = arg2.unwrap();
            let arg2_opt = arg2u.parse();
            if arg2_opt.is_err() {
                if map.contains_key(arg2u) {
                    arg2_val = *map.get(arg2u).unwrap();
                } else {
                    map.insert(arg2u.to_string().clone(), def_val);
                }
            } else {
                arg2_val = arg2_opt.unwrap();
            }
        }

        match cmd {
            // "snd" => last = arg1_val,
            "set" => *map.get_mut(arg1).unwrap() = arg2_val,
            "add" => *map.get_mut(arg1).unwrap() += arg2_val,
            "mul" => *map.get_mut(arg1).unwrap() *= arg2_val,
            "mod" => *map.get_mut(arg1).unwrap() %= arg2_val,
            "rcv" => {
                //if arg1_val != 0 {
                //    break;
                //}
                if rcvc.len() > 0 {
                    *map.get_mut(arg1).unwrap() = rcvc.pop_front().unwrap();
                } else {
                    *runningc = false;
                    return;
                }
            },
            "snd" => {
                rcvo.push_back(arg1_val);
                if def_val == 1 {
                    *pg1sends += 1
                }
                *runningo = true;
            },
            "jgz" => {
                if arg1_val > 0 {
                    *ind += arg2_val - 1;
                }
            }
            _ => println!("Error!"),
        }
        *ind += 1;
    }

}

fn main() {
    let mut f = File::open("day_18_input.txt").expect("File not found");
    let mut _inp = String::new();
    f.read_to_string(&mut _inp).expect("Something went wrong");
    let mut map0 = HashMap::new();
    let mut map1 = HashMap::new(); 

    let lines: Vec<&str> = _inp.trim().lines().collect();

    // let mut exec_ind: i32 = 0;
    // let mut last: i64 = 0i64;

    let mut ind0: i64 = 0;
    let mut ind1: i64 = 0;
    let mut running0 = true;
    let mut running1 = true;
    let mut pgm1_sends: u32 = 0;
    let mut rcv0 = VecDeque::new();
    let mut rcv1 = VecDeque::new();
    
    while running0 || running1 {

        run_prog(&lines, &mut map0, &mut ind0, 0i64, &mut running0, &mut running1, &mut rcv0, &mut rcv1, &mut pgm1_sends);
        run_prog(&lines, &mut map1, &mut ind1, 1i64, &mut running1, &mut running0, &mut rcv1, &mut rcv0, &mut pgm1_sends);
    }

    // println!("{:?}", last);
    println!("{:?}", pgm1_sends);

}
