/* Binary Search implementation in rust
 * Panic playground
 * Used vectors 
 *
 */

fn search(value:i32 , slice : Vec<i32> , head : i32 , tail : i32) ->bool {
    //println!("{} : {}" , head , tail);
    if head == tail {
        let index : usize = (head) as usize;
        return slice[index]==value;
    }
    else if head>tail || tail-head==1 {
        return false;
    }
    let mvalue : i32 = slice[(((head+tail)/2) as usize)];
    if value>mvalue {
        //search in top half
        let begin : i32 = (head+tail)/2;
        return search(value, slice , begin , tail);
    }
    else if value < mvalue{
        //search in bottom half
        let ending : i32 = (head+tail)/2;
        return search(value,slice , head , ending);
    }
    else{
        return true;
    }
} 


fn read_int() -> i32 {
    std::io::stdin().lines().next().unwrap().unwrap().parse().unwrap()
}

fn main() {
    println!("Enter n and then n numbers : ");
    let n : i32 = read_int();
    let mut nums : Vec<i32> = Vec::new();
    for _ in 0..n {
        nums.push(read_int());
    }
    println!("Now enter the number to look for : ");
    let search_for : i32 = read_int();
    let tail : i32 = (nums.len()-1) as i32; //if we did pass it in the below line , it would not work becs of
                                            //moving ownership to the search method (?)!
    let result : bool = search(search_for , nums , 0, tail);
    println!("{}{} found!" , search_for , if result {""} else {" not"});
}
