fn min_(a:usize , b:usize)->usize{
    if a<b {a} else {b}
}
fn count(a : &str , b : &str) -> i32 {
   let mut num : i32 = 0;
    for i in 0..b.len(){
        let mut is_ok = true;
        for j in i..min_(i+a.len(),b.len()){
            if a.chars().nth(j-i).unwrap() != b.chars().nth(j).unwrap(){
                is_ok = false;
            }
        }
        if is_ok && i+a.len()<=b.len(){
            num+=1;
        }
   } 
    num
}
fn read_line()-> String{
    std::io::stdin().lines().next().unwrap().unwrap().parse().unwrap()
}
fn main() {
    let a : String = read_line();
    let b : String = read_line();
    println!("{}",count(&b[..],&a[..]));



}

