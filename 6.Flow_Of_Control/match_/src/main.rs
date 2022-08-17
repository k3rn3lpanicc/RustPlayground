#[allow(dead_code)]
enum Color {
    // These 3 are specified solely by their name.
    Red,
    Blue,
    Green,
    // These likewise tie `u32` tuples to different names: color models.
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}




fn main() -> () {
    let number = 13;
    println!("Tell me about {}" , number);

    match number{
        1=> println!("One!"),
        2 | 3 | 5| 7| 11 => println!("This is a prime"),
        13..=19 => println!("A teen"),
        _=>println!("Ain't special"),
    }
    

    let boolean : bool = true;
    //match is also a expression!! so it can return values!
    let binary = match boolean{
        true => 1,
        false => 0,
    };
    println!("{} -> {}",boolean , binary);

    
    //TODO Destructuring : A match block can destructure items in a variety of ways.
    //TODO 1)Tuple:
    let triple = (0,-2,3);
    println!("Tell me about {:?}",triple);
    match triple{
        //Destructure second and third elements:
        (0,y,z) => println!("first is 0 , 'y' is {:?}, and 'z' is {:?}",y,z),
        (1,..) => println!("First is 1 and the rest doesn't matter!"),
        _   => println!("It doesn't matter what they are"),
        //'_' means don't bind the value to a variable
    }
    
    //TODO 2)Arrays/slices:
    let array = [1,-2,6];
    match array{
        [0,second,third] => 
            println!("lol {} , {}" , second , third),
        [1,_,third] =>
            println!("1 , ignored , {}" , third),
        [-1,second, ..]=>
            println!("-1, {} and all the other ones are ignored" , second),
        
		// Or store them in another array/slice (the type depends on
        // that of the value that is being matched against)
        [3, second, tail @ ..] => println!(
            "array[0] = 3, array[1] = {} and the other elements were {:?}",
            second, tail
        ),

        // Combining these patterns, we can, for example, bind the first and
        // last values, and store the rest of them in a single array
        [first, middle @ .., last] => println!(
            "array[0] = {}, middle = {:?}, array[2] = {}",
            first, middle, last
        ),
    }
    //TODO 3)enums:
	let color = Color::RGB(122, 17, 40);
    // TODO ^ Try different variants for `color`

    println!("What color is it?");
    // An `enum` can be destructured using a `match`.
    match color {
        Color::Red   => println!("The color is Red!"),
        Color::Blue  => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) =>
            println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) =>
            println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) =>
            println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) =>
            println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) =>
            println!("Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
                c, m, y, k),
        // Don't need another arm because all variants have been examined
    }

    //TODO 4)structs:

	struct Foo {
        x: (u32, u32),
        y: u32,
    }
    let foo = Foo { x: (1, 2), y: 3 };
	match foo {
        Foo { x: (1, b), y } => println!("First of x is 1, b = {},  y = {} ", b, y),

        // you can destructure structs and rename the variables,
        // the order is not important
        Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),

        // and you can also ignore some variables:
        Foo { y, .. } => println!("y = {}, we don't care about x", y),
        // this will give an error: pattern does not mention field `x`
        //Foo { y } => println!("y = {}", y),
    }
}
