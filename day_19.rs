use std::fs::File;
use std::io::Read;

fn valid(grid: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    return x < grid.len() && y < grid[0].len();
}


fn main() {
    let mut f = File::open("day_19_input.txt").expect("File not found");
    let mut _inp = String::new();
    f.read_to_string(&mut _inp).expect("Something went wrong");
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut seen: Vec<char> = Vec::new();
    
    let lines = _inp.lines();
    for line in lines {
        grid.push(line.chars().collect());
    }

    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut xdir: i32 = 1;
    let mut ydir: i32 = 0;
    let mut nsteps: i32 = 0;
    
    for i in 0..grid[0].len() {
        if grid[0][i] != ' ' {
           y = i as usize; 
        }
    }
    
    loop {
        let current = grid[x][y];
        nsteps += 1;

        if !(current == '|' || current == '-' || current == '+') {
            seen.push(current);
        }

        if current == '+' {
            // change direction
            for (dx, dy) in vec![(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let new_x = (dx + x as i32) as usize;
                let new_y = (dy + y as i32) as usize;
                if (dx, dy) != (-xdir, -ydir) && valid(&grid, new_x, new_y) && grid[new_x][new_y] != ' ' {
                    x = new_x;
                    y = new_y;
                    xdir = dx;
                    ydir = dy;
                }
            }
        } else {
            let new_x = (xdir + x as i32) as usize;
            let new_y = (ydir + y as i32) as usize;
            if grid[new_x][new_y] == ' ' {
                break;
            } else {
                x = new_x;
                y = new_y;
            }
        }
    }

    let ans: String = seen.iter().collect();
    println!("{:?}", ans);
    println!("{:?}", nsteps);

}
