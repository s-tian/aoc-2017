use std::fs::File;
use std::io::Read;

struct Rule {
    input: Vec<Vec<char>>,
    output: Vec<Vec<char>>,
}

fn rotate(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut rotated: Vec<Vec<char>> = Vec::new();
    for i in 0..grid.len() {
        let mut new_r: Vec<char> = Vec::new();
        for j in (0..grid.len()).rev() {
           new_r.push(grid[j][i]); 
        }
        rotated.push(new_r);
    }
    return rotated;
}

fn flipx(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut flipped: Vec<Vec<char>> = Vec::new();
    for i in (0..grid.len()).rev() {
        let mut new_r: Vec<char> = Vec::new();
        for j in 0..grid.len() {
           new_r.push(grid[i][j]); 
        }
        flipped.push(new_r);
    }
    return flipped;
}

fn flipy(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut flipped: Vec<Vec<char>> = Vec::new();
    for i in 0..grid.len() {
        let mut new_r: Vec<char> = Vec::new();
        for j in (0..grid.len()).rev() {
           new_r.push(grid[i][j]); 
        }
        flipped.push(new_r);
    }
    return flipped;
}

fn mat_eq(a: &Vec<Vec<char>>, b: &Vec<Vec<char>>) -> bool {

    for i in 0..a.len() {
        if a[i] != b[i] {
            return false;
        }
    }
    return true;
}

fn check_rotate(c: &Vec<Vec<char>>, rule: &Vec<Vec<char>>) -> bool {
    let mut b = c.clone();
    for _ in 0..3 {
        if mat_eq(&b, rule) {
            return true;
        }
        b = rotate(&b);
    }
    return false;
}

fn eq(rule: &Vec<Vec<char>>, pattern: &Vec<Vec<char>>) -> bool {

    if rule.len() != pattern.len() {
        return false;
    }
    let c = pattern.clone();
    return check_rotate(&c, rule) || check_rotate(&flipx(&c), rule) || check_rotate(&flipy(&c), rule);
}

fn main() {
    let mut f = File::open("day_21_input.txt").expect("File not found");
    let mut _inp = String::new();
    f.read_to_string(&mut _inp).expect("Something went wrong");

    let mut rules: Vec<Rule> = Vec::new();

    let lines = _inp.lines();
    for line in lines {
        let mut input: Vec<Vec<char>> = Vec::new();
        let mut output: Vec<Vec<char>> = Vec::new();
        let mut sections = line.split("=>");
        let inp_section = sections.next().unwrap().trim().split('/');
        for row in inp_section {
            input.push(row.chars().collect());
        }
        let out_section = sections.next().unwrap().trim().split('/');
        for row in out_section {
            output.push(row.chars().collect());
        }
        rules.push(Rule {input: input, output: output});
    }

    let mut grid: Vec<Vec<char>> = vec![vec!['.', '#', '.'], vec!['.', '.', '#'], vec!['#', '#', '#']]; 

    for _ in 0..18 {

        let mut new_grid: Vec<Vec<char>> = Vec::new();
        let sz: usize;
        if grid.len() % 2 == 0 {
            sz = 2;
        } else {
            sz = 3;
        }

        for i in 0..(grid.len()/sz) {
            for _ in 0..(sz+1) {
                new_grid.push(Vec::new());
            }
            let r = sz*i;
            for j in 0..(grid.len()/sz) {
                let c = sz*j;
                let mut sec: Vec<Vec<char>> = Vec::new();
                for m in 0..sz {
                    let mut t = Vec::new();
                    for n in 0..sz {
                        t.push(grid[r+m][c+n]);
                    }
                    sec.push(t);
                }
                for rule in &rules {
                    if eq(&rule.input, &sec) {
                        for r_pos in 0..(sz+1) {
                            for c_pos in 0..(sz+1) {
                                    new_grid[r+r_pos+i].insert(c+c_pos+j, rule.output[r_pos][c_pos]); 
                            }
                        }
                        break;
                    }
                }
            }
        }
        grid = new_grid;
    }

    let mut ans = 0;

    for row in grid {
        for c in row {
            if c == '#' {
                ans += 1;
            }
        }
    }

    println!("{:?}", ans);
}
