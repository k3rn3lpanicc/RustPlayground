#[macro_use]
extern crate scan_rules;



struct Point {
    x: f32,
    y: f32,
}

fn get_distance(p1:&Point , p2:&Point) -> f32 {
    ((p1.x-p2.x)*(p1.x-p2.x) + (p1.y-p2.y)*(p1.y-p2.y)).sqrt()
}


fn main() -> () {
    let mut points : Vec<Point> = Vec::new();
    for _i in 0..3{
        let result = try_readln!{
            (let x1:f32 , let y1:f32) => (x1,y1) 
        };     
        match result {
            Ok((x1,y1))=>{
               let p : Point = Point {x:x1,y:y1};
               points.push(p);
            }
            Err(_e)=>{
            }
        }   
    }
    let mut distances : Vec<f32> = Vec::new();
    for _i in 0..3{
        distances.push(get_distance(&points[_i],&points[(_i+1)%3]));
    }
    distances.sort_by(|a,b| a.partial_cmp(b).unwrap());
    if distances[0] == distances[1] || distances[1] == distances[2]{
        println!("Yes!");
    }
    else{
        println!("No!");
    }

}
/*  learned :
 *      - How to pass a object that does not implement the Copy trait as parameter to a function
 *      (with &)
 *      - How to sort a vector of floats (*)
 *      - How to define Structs and use them
 */

