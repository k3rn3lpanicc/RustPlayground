fn read_int() -> i32 {
    std::io::stdin().lines().next().unwrap().unwrap().parse().unwrap() // remember not to use ; in last return line
}

fn main() {
    println!("Please enter the size of numbers : ");
    let n : i32 = read_int();
    //let first_number : Vec<i32> = Vec::new();
    //let second_number : Vec<i32> = vec![]; //define vectors with macro! 
    let mut line = std::io::stdin().lines().next().unwrap().unwrap();
    let first_number : Vec<i32> = line.split(" ").map(|x| x.parse().expect("Not an integer!")).collect();
    //std::io::stdin().read_line(&mut line).expect("Failed to read line!");
    line = std::io::stdin().lines().next().unwrap().unwrap();
    let second_number : Vec<i32> = line.split(" ").map(|x| x.parse().expect("Not an integer!")).collect();
     
    //println!("first element : {}" , first_number[0]); // first_number[0] can also be written as first_number.get(0).unwrap()

    let mut result : Vec<i32> = Vec::new();
    let mut peymane : i32 = 0; //can also be written as 'let mut peymane = 0i32;'
    for i in 0..n {
        let index : usize = (n-i-1) as usize;
        let summage : i32  = first_number[index]  + second_number[index] + peymane;
        result.push(summage%10);
        peymane = summage/10;
    }
    if peymane!=0{
        result.push(peymane);
    }
    result.reverse();
    //println!("Result : {:?}" , result);
    print!("Result : ");
    for digit in result{
        print!("{}" , digit);
    }
    print!("\n");
}

/*
 * I leant these things : how to define vectors (macro way and normal way)
 * how to index vectors with for, and with indexes, (index must be usize type) so
 * I learned how to convert from an i32 to usize with 'as' keyword
 * learned to push values in vectors
 * learned to reverse the vector
 * and learned to read n numbers from input in one line (reading a line, spliting by ' ' then
 * mapping to int and collecting)!
 */
