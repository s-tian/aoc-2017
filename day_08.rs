use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use std::cmp;

fn main() {
    let mut f = File::open("day_8_input.txt").expect("File not found");
    let mut _inp = String::new();
    f.read_to_string(&mut _inp).expect("Something went wrong");

    let lines = _inp.lines();
    let mut map = HashMap::new();

    let mut highest: i32 = 0;
    
    for line in lines {
        let mut words = line.split_whitespace();
        let reg = words.next().unwrap();
        if !map.contains_key(reg) {
            map.insert(reg, 0);
        }
        let oper = words.next().unwrap();
        let mut operand: i32 = words.next().unwrap().parse().unwrap();
        if oper == "dec" {
            operand = -operand;
        }
        words.next(); //consume "if" 
        let cond_reg = words.next().unwrap();
        if !map.contains_key(cond_reg) {
            map.insert(cond_reg, 0);
        }
        let cond = words.next().unwrap();
        let cond_reg_val: i32 = *map.get(cond_reg).unwrap();
        let cond_comp_val: i32 = words.next().unwrap().parse().unwrap();
        let passed: bool; 
        match cond {
            "==" => passed = cond_reg_val == cond_comp_val,
            ">=" => passed = cond_reg_val >= cond_comp_val,
            ">" => passed = cond_reg_val > cond_comp_val,
            "<=" => passed = cond_reg_val <= cond_comp_val,
            "<" => passed = cond_reg_val < cond_comp_val,
            "!=" => passed = cond_reg_val != cond_comp_val,
            _ => passed = false,
        };

        if passed {
           *map.get_mut(reg).unwrap() += operand; 
           highest = cmp::max(highest, *map.get(reg).unwrap());
        }
    }
    // Part 1: 
    // let highest = map.values().max().unwrap();
    println!("{:?}", highest);

}
