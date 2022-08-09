/*  1)
 *      Sorting a vector of ints
 *  2)
 *      Sorting a vector of floats
 *  3)
 *      Sorting a vector of Structures
 */
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Person{
    name:String,
    age:u32
}




//New thing learned : implement methods for structures!!
impl Person{
    pub fn new(name:String , age:u32) -> Self{
        Person{
            name,
            age
        }
    }    
}

fn main() -> () {
    // 1) Int Vectod
    let mut vec : Vec<i32> = vec![5,1,10,2,15];
    vec.sort();
    assert_eq!(vec, vec![1, 2, 5, 10, 15]);
    // 2) Float Vector
    
    let mut vec2 : Vec<f32> = vec![1.1,1.15,5.5,1.123,2.0];
    vec2.sort_by(|a,b| a.partial_cmp(b).unwrap());
    assert_eq!(vec2, vec![1.1, 1.123, 1.15, 2.0, 5.5]);

    // 3) Struct Vector
    let mut vec3 : Vec<Person> = vec![Person::new("Zoe".to_string(), 25),
            Person::new("Al".to_string(), 60),
            Person::new("John".to_string(), 1),];
    // Sort people by derived natural order (Name and age)
    vec3.sort();
    assert_eq!(vec3,vec![Person::new("Al".to_string(), 60),
                           Person::new("John".to_string(), 1),
                           Person::new("Zoe".to_string(), 25),]);
    
    //Sort by age :
    vec3.sort_by(|a,b| b.age.cmp(&a.age));
    assert_eq!(vec3,vec![Person::new("Al".to_string(), 60),
                         Person::new("Zoe".to_string(), 25),
                         Person::new("John".to_string(), 1),]);
    
}













