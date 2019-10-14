use std::thread;
use std::sync::mpsc::channel;



pub fn start_heartbeat(speed :u32) {
    
    let (tx,rx) = channel();
    
    let sender = thread::spawn(move || {
        tx.send("Hello thread".to_owned())
            .expect("Unable to Send to channel");
    });

    let reciever = thread::spawn(move || {
        let value = rx.recv().expect("UnAble to recieve from channel");
    });
    sender.join().expect("the send tghread panicked");
    reciever.join().expect("the reciever thread has panicked");
}
pub fn end_heartbeat(){

}
pub fn beat(){
 println!("Beat from Child");
}