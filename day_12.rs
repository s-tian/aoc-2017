use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

fn parent(p: &i32, map: &HashMap<i32, i32>) -> i32 {
    if map.get(p).unwrap() == p {
        return *p;
    }
    return parent(map.get(p).unwrap(), map);
}

fn union(p: i32, q: i32, map: &mut HashMap<i32, i32>) {
    let p_par = parent(&p, &map);
    let q_par = parent(&q, &map);
    *map.get_mut(&q_par).unwrap() = p_par;
}

fn main() {
    let mut f = File::open("day_12_input.txt").expect("File not found");
    let mut _inp = String::new();
    f.read_to_string(&mut _inp).expect("Something went wrong");

    let lines = _inp.lines();
    let mut parents = HashMap::new();
    
    for line in lines {
        let mut words = line.split_whitespace();
        let p: i32 = words.next().unwrap().parse().unwrap();
        if !parents.contains_key(&p) {
            parents.insert(p, p);
        }  
        words.next(); // consume '<->'
        for comp in words {
            let q: i32 = comp.trim_matches(',').parse().unwrap();
            if !parents.contains_key(&q) {
                parents.insert(q, q);
            }
            union(p, q, &mut parents);
        }
    }
    let mut p1_ans: i32 = 0;
    let mut group_parents: Vec<i32> = Vec::new();

    let parent_0 = parent(&0, &parents);
    
    for (_id, par) in &parents {
        let p =  parent(par, &parents);
        if p == parent_0 { 
            p1_ans += 1;
        }  
        if !group_parents.contains(&p) {
            group_parents.push(p);
        }
    }
    println!("{:?}", p1_ans);
    println!("{:?}", group_parents.len());
}
