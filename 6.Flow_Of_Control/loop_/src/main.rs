/*  -loops
 *  -return from loop with value (let with loop)
 *  -nested loops with label
 */
//This is for allowing the code to be run (a println is unreacbable)!
#![allow(unreachable_code)]

fn main() -> (){
    //1)loops
    let mut count : i32 = 0;
    loop{
        count+=1;
        if count==3 {
            println!("Breaking out of loop");
            break;
        }
    }
    
    //2)loops with labels:
	'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // This would break only the inner loop
            //break;

            // This breaks the outer loop
            break 'outer;
        }

        println!("This point will never be reached");
    }
    println!("Exited the outer loop");
    

    //3)Returning from loops with value
    let mut counter : i32 = 0;
    let result = loop{
        counter+=1;
        if counter == 10 {
            break counter*2;
        }
    };
    println!("Result : {}" , result);

}
