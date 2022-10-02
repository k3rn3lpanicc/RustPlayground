use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx , rx) = mpsc::channel();
    let t1 = thread::spawn(move ||{
        thread::sleep(Duration::from_secs(2));
        /*
            The transmitter has a send method that takes the value we want to send. The send method 
            returns a Result<T, E> type, so if the receiver has already been dropped and there’s nowhere to send a value,
            the send operation will return an error.
        */
        tx.send("lol").unwrap();
    });
    
    /*
        The receiver has two useful methods:   recv and try_recv.
        
        We’re using recv, short for receive, which will block the main thread’s execution and wait until a value is
        sent down the channel. Once a value is sent, recv will return it in a Result<T, E>. When the transmitter 
        closes, recv will return an error to signal that no more values will be coming.

        The try_recv method doesn’t block, but will instead return a Result<T, E> immediately: an Ok value holding a message if one is available
        and an Err value if there aren’t any messages this time. Using try_recv is useful if this thread has other work to do while waiting 
        for messages: we could write a loop that calls try_recv every so often, handles a message if one is available, and otherwise does other
        work for a little while until checking again.
    */
    println!("{}", rx.recv().unwrap()); //this part will be blocked for 2 seconds and then print lol


                                        // Creating Multiple Producers by Cloning the Transmitter


    let (tx , rx) = mpsc::channel();
    let tx1 = tx.clone();
    /*
        This time, before we create the first spawned thread, we call clone on the transmitter. This will give us a new transmitter 
        we can pass to the first spawned thread. We pass the original transmitter to a second spawned thread. This gives us two threads, 
        each sending different messages to the one receiver.
    */
    let t2 = thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    let t3 = thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    /*
        In the main thread, we’re not calling the recv function explicitly anymore: instead, we’re treating rx as an iterator. 
        For each value received, we’re printing it. When the channel is closed, iteration will end.
    */
    for received in rx {
        println!("Got: {}", received);
    }


    t1.join().unwrap();
    t2.join().unwrap();
    t3.join().unwrap();
}
