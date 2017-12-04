use std::fs::File;
use std::io::Read;

fn main() {
    let mut f = File::open("day_1_input.txt").expect("File not found");
    let mut _inp = String::new();
    f.read_to_string(&mut _inp).expect("Something went wrong");
    let mut total = 0;
    for i in 0.._inp.len() { 
        let a = _inp.chars().nth(i).unwrap();
        let b = _inp.chars().nth((i+_inp.len()/2)%_inp.len()).unwrap();
        if a == b {
            total = total + a.to_digit(10).unwrap();
        }
    }   
    println!("{}", total);
}
