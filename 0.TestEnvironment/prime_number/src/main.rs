fn is_prime(a:i32) -> bool {
    if a==1{
        return false;
    }
    for i in 2..a {
        if a%i==0{
            return true;
        }
    }
    return false;
}

fn read_int() -> i32 {
    std::io::stdin().lines().next().unwrap().unwrap().parse().unwrap()
}

fn main() -> () {
    let a : i32 = read_int();
    println!("{} is{} prime" , a , if !is_prime(a) {""} else {" not"});
} 



