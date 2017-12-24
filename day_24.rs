use std::fs::File;
use std::io::Read;
use std::cmp;

struct Component {
    port1: u32,
    port2: u32,
}

fn sum(bridge: &Vec<(usize, bool)>, components: &Vec<Component>) -> u32 {
    let mut s = 0;
    for &(index, _) in bridge {
        let comp = &components[index];
        s += comp.port1 + comp.port2;
    }
    return s;
}

fn in_bridge(bridge: &Vec<(usize, bool)>, comp: usize) -> bool {
    for &(ind, _) in bridge {
        if ind == comp {
            return true;
        }
    }
    return false;
}

// Represent bridges as vectors of tuples, where the first values
// corresponds to the index of the component and the second is 
// false if it is in the original orientation and true if flipped

fn build(bridge: &Vec<(usize, bool)>, components: &Vec<Component>) -> (u32, u32) {
    let last_section = bridge[bridge.len()-1];
    let last_comp_ind = last_section.0;
    let last_comp = &components[last_comp_ind]; 
    let open: u32;
    if last_section.1 {
        open = last_comp.port1;
    } else {
        open = last_comp.port2;
    }
    let mut max_val = 0;
    let mut max_len = 0;
    for i in 0..components.len() {
        if components[i].port1 == open || components[i].port2 == open {
            if !in_bridge(&bridge, i) {
                let mut ext_bridge = bridge.clone();
                if components[i].port1 == open {
                    ext_bridge.push((i, false));
                } else {
                    ext_bridge.push((i, true));
                }
                // max_val = cmp::max(max_val, build(ext_bridge, components));
                let (val, len) = build(&ext_bridge, components);
                if len > max_len {
                    max_len = len;
                    max_val = val;
                } else if len == max_len {
                    max_val = cmp::max(max_val, val);
                }
            }
        }
    }
    let current_sum = sum(&bridge, components);
    if max_len == 0 {
        return (current_sum, bridge.len() as u32);
    }
    return (max_val, max_len);
    // return cmp::max(max_val, current_sum);
}

fn main() {
    let mut f = File::open("day_24_input.txt").expect("File not found");
    let mut _inp = String::new();
    f.read_to_string(&mut _inp).expect("Something went wrong");

    let lines = _inp.lines();
    let mut components: Vec<Component> = Vec::new();
    components.push(Component {port1: 0, port2: 0});
    for line in lines {
        let mut nums = line.split('/');
        let p1: u32 = nums.next().unwrap().parse().unwrap();
        let p2: u32 = nums.next().unwrap().parse().unwrap();
        components.push(Component {port1: p1, port2: p2});
    }

    // Part 1
    // let best = build(vec![(0, false)], &components);

    // println!("{:?}", best);

    // Part 2
    let (val, _len) = build(&vec![(0, false)], &components);
    println!("{:?}", val);
}
