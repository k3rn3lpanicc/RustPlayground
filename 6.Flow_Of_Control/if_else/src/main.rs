/*  if/else
 *  use if/else with let!
 */
fn main() -> (){
    let a : i32 = 5;
    if a==5{
        println!("Yes");
    }

    let b : i32 = 
        if a<5 {
            println!("less than 5");
            10
        }
        else{
            println!("more than or equals to 5");
            20
        }; // Do not forget this semicolon here!!
    println!("{}" , b);     
}
