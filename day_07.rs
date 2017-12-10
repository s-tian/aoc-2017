use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

fn balance(node: &str, tree: &HashMap<&str, Vec<String>>, weights: &HashMap<&str, i32>) -> i32 {
    let mut w: Vec<i32> = Vec::new();
    for branch in tree.get(node).unwrap() {
        let a = balance(branch, tree, weights);
        if a != 0 { // If the error occurs at a higher branch, return it
            return a; 
        }
        w.push(weight(branch, tree, weights));
    }
    w.sort();
    if w.len() > 0 && w[0] != w[w.len()-1 as usize] {
        let correct = w[1];
        let incorrect;
        if w[0] != correct {
           incorrect = w[0]; 
        } else {
            incorrect = w[w.len()-1];
        }
        for branch in tree.get(node).unwrap() {
            if weight(branch, tree, weights) == incorrect {
                return *weights.get::<str>(branch).unwrap() + correct - incorrect;
            }
        }
    }
    return 0;
}

fn weight(node: &str, tree: &HashMap<&str, Vec<String>>, weights: &HashMap<&str, i32>) -> i32 {
    let mut total:i32 = *weights.get(node).unwrap();
    for branch in tree.get(node).unwrap() {
       total += weight(branch, tree, weights); 
    }
    return total;
}

fn main() {
    let mut f = File::open("day_7_input.txt").expect("File not found");
    let mut _inp = String::new();
    f.read_to_string(&mut _inp).expect("Something went wrong");

    let lines = _inp.lines();
    let mut tree = HashMap::new();
    let mut weights = HashMap::new();
    let mut children: Vec<String> = Vec::new();
    for line in lines { 
        let mut words = line.split_whitespace();
        let prog = words.next().unwrap();
        let weight: i32 = words.next().unwrap().to_string().trim_matches(|c| c=='(' || c==')').parse().unwrap();
        if words.next() == None {
            tree.insert(prog, Vec::new());
        } else {
            let mut c = words.next();
            let mut nodes: Vec<String> = Vec::new();
            while !(c == None) {
                nodes.push(c.unwrap().to_string().trim_matches(',').to_string());
                children.push(c.unwrap().to_string().trim_matches(',').to_string());
                c = words.next();
            } 
            tree.insert(prog, nodes);
        }
        weights.insert(prog, weight);
    }

    // Part 1: Find root
    let mut root: &str = "none";
    for (key, _value) in &tree {
        let mut found = false;
        for child in &children {
            if key == child {
                found = true;
                break;
            }
        }
        if !found {
            root = key;
            break;
        }
    }

    // Part 2: Find incorrect weight 

    println!("{:?}", balance(root, &tree, &weights));

}
