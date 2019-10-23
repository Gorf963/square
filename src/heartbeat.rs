use std::thread;
use std::time;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};




pub fn start_heartbeat(speed :u64) -> mpsc::Sender<String> {
    

    let (heartbeat_control_tx,heardbeat_control_rx) = mpsc::channel();

    //Set speed to millseconds
    let speed: u64 = 1000*60/speed;
    
    thread::spawn(move || {
        beat(speed, heardbeat_control_rx);
    });
    return heartbeat_control_tx;
}
pub fn end_heartbeat(heartbeat_control_tx: Sender<String>){
    heartbeat_control_tx.send(format!("End")).expect("Error Sending");
}
fn beat(speed :u64, rx: Receiver<String>){
  loop 
  {
    println!("Beat from Child");
    thread::sleep(time::Duration::from_millis(speed));
    let message = rx.try_recv();
    if message.is_ok() {
        if message.unwrap() == "End" {
            break;
        }
    }
  }
}
