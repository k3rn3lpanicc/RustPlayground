fn read_line() -> String{
    std::io::stdin().lines().next().unwrap().unwrap()
}
fn check_validation(st : &str) -> bool {
    let mut a : i32 = 0;
    for i in 0..(String::from(st).len()){
        if st.chars().nth(i).unwrap()==')' {
            a-=1;
        }
        else{
            a+=1;
        }
        if a<0{
            return false;
        }
    }
    print!("{}",if a==0 {st} else{""});
    return a==0;
}
fn main() -> () {
    let mut max : i32 = 0;
    let st : String = read_line();
    let mut ind : i32 = 0;
    while ind < st.len() as i32 {
        let mut len : i32 = 1;
        while len<(st.len() as i32)-ind+1{
            if check_validation(&st[(ind as usize)..((ind+len)as usize)]){
                print!("\n");
                if max<len{
                    max = len;
                }
            }
            len+=1;
        }        
        ind+=1;
    }
    println!("Max : {}",max);
}
/*
 * Learned : array slice
 * String indexing
 * use usize as index
 * st.chars().nth(i).unwrap()!!!
 */
