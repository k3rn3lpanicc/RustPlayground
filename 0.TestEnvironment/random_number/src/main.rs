use rand::Rng;

fn read_int() -> i32 {
    std::io::stdin().lines().next().unwrap().unwrap().parse().unwrap()
}
fn abs(a:i32)->i32{
    if a>0 {a} else {-a}
}
fn main() -> () {
    let mut rng = rand::thread_rng();
    let n : i32 = read_int();
    let m : i32 = read_int();
    let r : i32 = abs(rng.gen())%(m-n)+n;
    println!("{}" , r); 
}
/*
 *What I've learned :
 *      - rand = "0.8.5" crate for random number generation
 *      - generate random numbers!
 */
