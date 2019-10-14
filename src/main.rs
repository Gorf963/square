mod pause;
mod heartbeat;

use pause::pause;
use std::io;
use heartbeat::{start_heartbeat, end_heartbeat};


fn main() {
    
    setup();
    pause();
    end_heartbeat();
}

fn setup() {
    let mut beat = String::new();

    println!("Please enter the number of seconds between heartbeats" );
    
    io::stdin().read_line(&mut beat)
        .expect("Error in reading line");

    let beat: u32 = beat.trim().parse()
        .expect("Not a number");
    start_heartbeat(beat);
}