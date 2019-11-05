use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};

mod square {
    pub struct Square {
        value: i32,
        pump: i32,
        heartbeat_rx mpsc::Receiver<String>,
        north_tx mpsc::Sender<message>,
        north_rx mpsc::Receiver<message>,
        south_tx mpsc::Sender<message>,
        south_rx mpsc::Receiver<message>,
        east_tx mpsc::Sender<message>,
        east_rx mpsc::Receiver<message>,
        west_tx mpsc::Sender<message>,
        west_rx mpsc::Receiver<message>,
        
    }
    pub struct message {
        value: i32;
        player: i8;
    }
}


