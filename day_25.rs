use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

fn main() {
    let mut f = File::open("day_25_input.txt").expect("File not found");
    let mut _inp = String::new();
    f.read_to_string(&mut _inp).expect("Something went wrong");

    fn get_char(s: &str) -> char {
        return s.chars().nth(0).unwrap();
    }

    let mut lines = _inp.trim().lines();
    let mut curr_state: char = get_char(lines.next().unwrap().split(' ').nth(3).unwrap());
    let num_steps: u32 = lines.next().unwrap().split(' ').nth(5).unwrap().parse().unwrap();
    let mut state_trans = HashMap::new();
    let mut tape = HashMap::new();
    let mut tape_ind = 0;
    let mut checksum = 0;

    while !(lines.next() == None) {
        let start_state = get_char(lines.next().unwrap().split(' ').nth(2).unwrap());
        for i in 0..2 {
            lines.next(); // Discard first line
            let assign = get_char(lines.next().unwrap().trim().split(' ').nth(4).unwrap()).to_digit(10).unwrap();
            let dir = lines.next().unwrap().trim().split(' ').nth(6).unwrap();
            let mut dir_num = 0;
            match dir {
                "right." => dir_num = 1,
                "left." => dir_num = -1,
                _ => println!("Error"),
            }
            let next_state = get_char(lines.next().unwrap().trim().split(' ').nth(4).unwrap());
            state_trans.insert((start_state, i), (assign, dir_num, next_state));
        }
    }
    
    for _i in 0..num_steps {
        if !tape.contains_key(&tape_ind) {
            tape.insert(tape_ind, 0);
        }
        let curr_val = *tape.get(&tape_ind).unwrap();
        let oper = state_trans.get(&(curr_state, curr_val)).unwrap();
        if curr_val != oper.0 {
            if curr_val == 1 {
                checksum -= 1;
            } else {
                checksum += 1;
            }
        }

        *tape.get_mut(&tape_ind).unwrap() = oper.0;
        tape_ind += oper.1;
        curr_state = oper.2;
    }
    println!("{:?}", checksum);
}
