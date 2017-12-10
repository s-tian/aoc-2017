
fn map_2d(x: i32, y: i32) -> usize{
   return (x+100*y) as usize; 
}

fn update(arr: &mut [i32], x: i32, y: i32) -> i32 {
   arr[map_2d(x, y)] = arr[map_2d(x-1, y)] + arr[map_2d(x+1, y)] +
                    arr[map_2d(x, y-1)] + arr[map_2d(x, y+1)] +
                    arr[map_2d(x-1, y-1)] + arr[map_2d(x-1, y+1)] +
                    arr[map_2d(x+1, y-1)] + arr[map_2d(x+1, y+1)]; 
   return arr[map_2d(x, y)];
}

fn main() {


    let _target = 347991;
    // let mut x:i32 = 0;
    // let mut y:i32 = 0;
    let mut x:i32 = 50;
    let mut y:i32 = 50;
    let mut memo = [0; 10000];
    let mut i = 1;
    memo[map_2d(x, y)] = 1;
    let mut step = 2;
    let mut r = 1;
    while r < _target {
        x = x + 1;
        i = i + 1;
        r = update(&mut memo, x, y);
        if r > _target {
            //println!("{:?}", x.abs() + y.abs());
            println!("{:?}", r);
        }
        for _ in 0..step-1 {
            y = y - 1; 
            i = i + 1;
            r = update(&mut memo, x, y);
            if r > _target {
                //println!("{:?}", x.abs() + y.abs());
                println!("{:?}", r);
            }
        }
        for _ in 0..step {
            x = x - 1; 
            i = i + 1;
            r = update(&mut memo, x, y);
            if r > _target {
                //println!("{:?}", x.abs() + y.abs());
                println!("{:?}", r);
            }
        }
        for _ in 0..step {
            y = y + 1; 
            i = i + 1;
            r = update(&mut memo, x, y);
            if r > _target {
                //println!("{:?}", x.abs() + y.abs());
                println!("{:?}", r);
            }

        }
        for _ in 0..step {
            x = x + 1; 
            i = i + 1;
            r = update(&mut memo, x, y);
            if r > _target {
                //println!("{:?}", x.abs() + y.abs());
                println!("{:?}", r);
            }

        }
        step = step + 2;
    }
}
