#[macro_use]
extern crate scan_rules;

fn is_perpendicular(a:i32,b:i32,c:i32) -> bool {
    //We can use expression instead of return statement!! an expression should not end with ;
    a*a+b*b==c*c || b*b+c*c==a*a || a*a+c*c==b*b
}

fn _read_int()->i32 {
    return std::io::stdin().lines().next().unwrap().unwrap().parse().unwrap();
}

fn main() -> () {
    println!("Please enter numbers a b c, like 3 4 5 : ");
    let result = try_readln! {
        (let a: i32 , let b : i32 , let c : i32) => (a,b,c)
    };
    match result {
        Ok((a,b,c)) => {
            println!("The triangle is {} perpendicular!" , if is_perpendicular(a,b,c) {""} else {"not"});
        },
        Err(e) => {
            println!("Error : {}" , e);
        },
    }
    /*
    let a : i32 = read_int();
    let b : i32 = read_int();
    let c : i32 = read_int();
    let d : bool = is_perpendicular(a,b,c);
    println!("\nThe triangle is {} perpendicular!" , (if d {""} else {"not"}));
    */
}



