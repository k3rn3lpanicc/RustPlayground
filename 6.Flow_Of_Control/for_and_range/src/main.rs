/*  <For loops and range>
 *  1-range
 *  2-inclusive range
 *  3-iterators
 */


fn main() -> () {
    //1)range
    for n in 1..101 {   //n gets these values : 1,2,3,4,...,100
        if n%15 == 0{
            println!("Fizzbuzz");
        }
        else if n%3 == 0 {
            println!("Fizz");
        }
        else if n%5 == 0 {
            println!("Buzz");
        }
        else{
            println!("{}" , n);
        }
    }
    
    //2)inclusive range
    
    for n in 1..=100 { //n also gets values : 1,2,3,4,...,100
        if n%15 == 0{
            println!("FizzBuzz");
        }
        else if n%5 == 0{
            println!("Buzz");
        }
        else if n%3 == 0 {
            println!("Fizz");
        }
        else{
            println!("{}" , n);
        }
    }
    
    //3)iterators
    
    /*
     * The for in construct is able to interact with an Iterator in several ways. As discussed in
     * the section on the Iterator trait, by default the for loop will apply the into_iter function
     * to the collection. However, this is not the only means of converting collections into
     * iterators.
     *
     * into_iter, iter and iter_mut all handle the conversion of a collection into an iterator in
     * different ways, by providing different views on the data within.
     */
    
    /*      1)iter - This borrows each element of the collection through each iteration. Thus leaving the
     *  collection untouched and available for reuse after the loop.
     */
    
	let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match *name { //TODO name is &&str so *name is &str and also "Ferris" is &str so they are same type TODO
            "Ferris" => println!("There is a rustacean among us!"), //i could also use "Ferris" as &"Ferris" and put name instead of *name in match!
            // TODO ^ Try deleting the & and matching just "Ferris"
            _ => println!("Hello {}", name),
        }
    }
    
    println!("names: {:?}", names);
    
    /*      2)into_iter - This consumes the collection so that on each iteration the exact data is
     *  provided. Once the collection has been consumed it is no longer available for reuse as it
     *  has been 'moved' within the loop.
     */
    
	let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    
    //println!("names: {:?}", names); //This line will make a compile error
    
    /*      3)iter_mut - This mutably borrows each element of the collection, allowing for the
     *  collection to be modified in place.
     */
  	let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names);
	  

}
