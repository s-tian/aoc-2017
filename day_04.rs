use std::fs::File;
use std::io::Read;

fn anagram(a: &str, b: &str) -> bool {
    let mut _a = a.chars().collect::<Vec<char>>();
    _a.sort();
    let a_sorted: String = _a.into_iter().collect::<String>();
    let mut _b = b.chars().collect::<Vec<char>>();
    _b.sort();
    let b_sorted: String = _b.into_iter().collect::<String>();
    return a_sorted == b_sorted;
}

fn valid(msg: &str) -> bool { 
    let mut pass: Vec<&str> = Vec::new();
    let words = msg.split_whitespace();
    for word in words {
        for existing in &pass {
            // if word == existing.to_string() {
            if anagram(&word, existing) {
                return false;
            }
        }
        pass.push(&word);
    }
    return true;
}

fn main() {
    let mut f = File::open("day_4_input.txt").expect("File not found");
    let mut _inp = String::new();
    f.read_to_string(&mut _inp).expect("Something went wrong");

    let lines = _inp.lines();
    let mut total:i32 = 0;
    for line in lines {
        if valid(&line) {
            total = total + 1;
        }
    }
    println!("{:?}", total);

}
