use std::thread;
use std::sync::mpsc; // Channels
use rand::Rng;

fn main() {
    using_channels()  ;  
}

fn simle_thread()
{
      // Give a thread a closure
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
        }
    });

    // Here code can be executed on the main thread

    // Join waits for thread to complete
    handle.join().unwrap();
 }

 fn using_channels()
 {
     // Create my sender and reciever (just 1 of each)
    let (tx, rx) = mpsc::channel();
    let val = "She turned me into a";
    let types = ["newt", "moster", "human"];

    //let tx1 = mpsc::Sender::clone(&tx);
   
    thread::spawn(move || { 
        let secret_number = rand::thread_rng().gen_range(0, 3);
        let message = format!("{} {}", val, &types[secret_number]);    
        tx.send(message).unwrap(); // message dies here
    });

    //....

    for message in rx {
         println!("{}", message);
         if message.contains("newt")
         {
              println!("but I got better again");
         }
     }
 }