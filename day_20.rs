use std::fs::File;
use std::io::Read;

struct Particle {
    x: Vec<i32>,
    y: Vec<i32>,
    z: Vec<i32>,
    destroyed: bool,
}

fn main() {
    let mut f = File::open("day_20_input.txt").expect("File not found");
    let mut _inp = String::new(); 
    f.read_to_string(&mut _inp).expect("Something went wrong");

    let lines = _inp.lines();
    /*
    let mut ans = 0;
    let mut lowest = -1; 
    let mut id = 0;
    */
    let mut particles: Vec<Particle> = Vec::new();

    for line in lines {
        let mut parser = line.split(|c| c == '<' || c== '>');
        let mut x: Vec<i32> = Vec::new();
        let mut y: Vec<i32> = Vec::new();
        let mut z: Vec<i32> = Vec::new();
        
        for _ in 0..3 {
            parser.next();
            let mut comp_iter = parser.next().unwrap().split(',');
            x.push(comp_iter.next().unwrap().parse::<i32>().unwrap());
            y.push(comp_iter.next().unwrap().parse::<i32>().unwrap());
            z.push(comp_iter.next().unwrap().parse::<i32>().unwrap());
        } 
        // println!("{:?}", y);

        particles.push(Particle {x: x, y: y, z: z, destroyed: false});

        // Part 1
        // Compute the minimum of the sums of the squares of acceleration terms
        /*
        let acc_iter = parser.next().unwrap().split(',');
        let mut acc_term = 0;
        for comp in acc_iter {
            acc_term += comp.parse::<i32>().unwrap().pow(2);
        }
        if acc_term < lowest || lowest == -1 {
            ans = id;
            lowest = acc_term;
        }
        
        id += 1;
        */
    }

    for tick in 0..1000{
        for particle in particles.iter_mut() {
            if !particle.destroyed {
                particle.x[1] += particle.x[2];
                particle.y[1] += particle.y[2];
                particle.z[1] += particle.z[2];

                particle.x[0] += particle.x[1];
                particle.y[0] += particle.y[1];
                particle.z[0] += particle.z[1];
            }
        }
        for i in 0..particles.len() { 
            let mut found = false;
            let x = particles[i].x[0];
            let y = particles[i].y[0];
            let z = particles[i].z[0];
            for j in 0..particles.len() {
                if i != j && particles[j].x[0] == x && particles[j].y[0] == y && particles[j].z[0] == z {
                    particles[j].destroyed = true;
                    found = true;
                }

            }
            if found {
                particles[i].destroyed = true;
            }
        }
    }


    let mut remaining = 0;

    for particle in &particles {
        if !particle.destroyed {
            remaining += 1;
        }
    }
    
    // println!("{:?}", ans);
    println!("{:?}", remaining);

}
