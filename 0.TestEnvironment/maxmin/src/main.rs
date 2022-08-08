/*Minmax
 *Simple rust training with university problems
 *
 *
 */
#[macro_use]
extern crate scan_rules;



fn main() {
    let mut numbers : Vec<i32> = Vec::new();
    let result = try_readln! {
        (let a:i32 , let b:i32 , let c:i32 , let d:i32 , let e:i32) => (a,b,c,d,e)
    };
    match result {
        Ok((a,b,c,d,e)) => {
            numbers.push(a);
            numbers.push(b);
            numbers.push(c);
            numbers.push(d);
            numbers.push(e);
        },
        Err(e) => {
            println!("Error : {}" , e);
        },
    }
    let mut min : i32 = numbers[0];
    let mut max : i32 = numbers[0];
    for i in 1..5{
        let a : i32 = numbers[((i) as usize)];
        if a<min{
            min = a;
        }
        if a>max{
            max = a;
        }
    }
    println!("{}",max-min);

}

