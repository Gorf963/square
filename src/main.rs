mod pause;
mod heartbeat;

use pause::pause;
use std::io;
use std::sync::mpsc;
use heartbeat::{start_heartbeat, end_heartbeat};


fn main() {
    
    let heartbeat_sender: mpsc::Sender<String>;

    
    heartbeat_sender = setup();
    pause();
    end_heartbeat(heartbeat_sender);
    println!("Ended Normal");
}

fn setup()  -> mpsc::Sender<String> {
    let mut beat = String::new();

    println!("Please enter the number of heartbeats per min" );
    
    io::stdin().read_line(&mut beat)
        .expect("Error in reading line");
    let beat: u64 = beat.trim().parse()
        .expect("Not a number");
    let tx = start_heartbeat(beat);
    return tx;
}