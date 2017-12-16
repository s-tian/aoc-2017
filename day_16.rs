use std::fs::File;
use std::io::Read;

fn spin(v: &mut Vec<char>, arg: usize) -> Vec<char>  { 
    return [&v[(v.len()-arg)..], &v[..(v.len()-arg)]].concat();
}

fn main() {
    let mut f = File::open("day_16_input.txt").expect("File not found");
    let mut _inp = String::new();
    f.read_to_string(&mut _inp).expect("Something went wrong");

    let mut seen: Vec<Vec<char>> = Vec::new();
    let mut pgms: Vec<char> = (0..16).map(|x| (x + 'a' as u8) as char).collect();

    while !seen.iter().any(|ref a| a == &&pgms) {

        seen.push(pgms.clone());
        let cmds = _inp.split(',');

        for cmd in cmds {
            let cmd_type = cmd.chars().nth(0).unwrap();
            let arg_exp = &cmd[1..].to_string();
            let mut args = arg_exp.split('/');
            match cmd_type {
                's' => pgms = spin(&mut pgms, args.next().unwrap().parse::<u32>().unwrap() as usize), 
                'x' => {
                    let arg1 = args.next().unwrap().parse::<u32>().unwrap(); 
                    let arg2 = args.next().unwrap().parse::<u32>().unwrap();
                    pgms.swap(arg1 as usize, arg2 as usize);
                },

                'p' => {
                    let (arg1, arg2) = (args.next().unwrap().chars().next().unwrap(), args.next().unwrap().chars().next().unwrap());
                    let p1 = pgms.iter().position(|&s| s == arg1).unwrap();
                    let p2 = pgms.iter().position(|&s| s == arg2).unwrap();
                    pgms.swap(p1, p2);
                }
                _ => println!("Something went wrong")
            }
        }
    }    

    // let ans: String = pgms.into_iter().collect();
    // println!("{:?}", ans);
    
    let cycle_pos = seen.iter().position(|ref s| s == &&pgms).unwrap();
    let cycle_len = seen.len()-cycle_pos;
    
    let billion_dances = &seen[cycle_pos + ((1000000000 - cycle_pos) % cycle_len)]; 
    let ans: String = billion_dances.into_iter().collect();
    println!("{:?}", ans);

}
