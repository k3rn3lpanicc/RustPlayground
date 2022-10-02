use std::{thread, time::Duration};

fn main() {
    //create an inline thread, here we have access to main variables
    let first_thread = thread::spawn(||{
        for i in 1..10{
            println!("lol : {}" , i);
            thread::sleep(Duration::from_secs(1));  
        }
    });
    
    //create a thread which calls a function (outside of main)
    let second_thread = thread::spawn(||{
        foo();
    });
    
    //main thread
    for i in 1..5{
        println!("not lol : {}" , i);
    }

    //wait for the threads to finish (if you miss these, you program will end before threads get a chance to run)
    first_thread.join().unwrap();
    second_thread.join().unwrap();
}

fn foo() {
    println!("Hello, world!");
    thread::sleep(Duration::from_secs(10));
    println!("Goodbye, world!");
}
