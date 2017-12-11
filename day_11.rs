use std::fs::File;
use std::io::Read;
use std::cmp;

fn min_dist(mut straight: i32, mut east: i32, mut west: i32) -> i32 {
    if east*west > 0 {
        let overlap: i32 = cmp::min(east.abs(), west.abs())*east/east.abs();    
        straight += overlap; 
        east -= overlap;
        west -= overlap;
    }

    if east*straight < 0 {
        let overlap: i32 = cmp::min(east.abs(), straight.abs())*straight/straight.abs();
        straight -= overlap;
        east += overlap;
        west += overlap;
    }

    if west*straight < 0 {
        let overlap: i32 = cmp::min(west.abs(), straight.abs())*straight/straight.abs();
        straight -= overlap;
        west += overlap;
        east += overlap;
    }

    return straight.abs() + west.abs() + east.abs();
}

fn main() {
    let mut f = File::open("day_11_input.txt").expect("File not found");
    let mut _inp = String::new();
    f.read_to_string(&mut _inp).expect("Something went wrong");

    let directions: Vec<&str> = _inp.trim().split(",").collect();
    
    let mut straight: i32 = 0;
    let mut east: i32 = 0;
    let mut west: i32 = 0;
    let mut max_dist: i32 = 0; 

    for dir in directions {
        match dir {
            "n" => straight += 1, 
            "nw" => west += 1,
            "ne" => east += 1,
            "s" => straight -= 1,
            "se" => west -= 1,
            "sw" => east -= 1,
            _ => println!("Unknown dir"),
        }
        max_dist = cmp::max(max_dist, min_dist(straight, west, east));     
    }

    // Part 1
    println!("{:?}", min_dist(straight, west, east)); 
    // Part 2
    println!("{:?}", max_dist);

}
