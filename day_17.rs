fn main() {
    let _inp: usize = 382;
    let mut buf: Vec<i32> = Vec::new();
    buf.push(0);
    let mut pos: usize = 0;

    for i in 1..2018 {
        pos = ((pos + _inp) % buf.len()) + 1;
        buf.insert(pos, i);
    }
    // Part 1
    println!("{:?}", buf[pos+1]);
    
    let mut pos2: usize = 0;
    let mut buf1: u32 = 0;
    for i in 1..50000001 {
        pos2 = (pos2 + _inp) % i as usize + 1; 
        if pos2 == 1 {
           buf1 = i; 
        }
    }
    // Part 2
    println!("{:?}", buf1);
}
