fn read_line() -> String {
    std::io::stdin().lines().next().unwrap().unwrap()
}

fn read_int() -> i32 {
    read_line().parse().unwrap()
}

fn main() {
    println!("Enter n : ");
    let n : i32 = read_int();
    let mut m : i32 = 0;
    let mut sum : i32 = 0;
    for _i in 0..n {
        let mut x : i32 = read_int();
        if x>=8 {
            if x>=10 && x<=15 {
                x+=1;
            }
            sum+=x;
            m+=1;
        }
    }
    let result : f32 = ((sum) as f32)/((m) as f32);
    println!("Result : {}" , result);
}
