// User input (keyboard) thread.
use std::io::{self, Write};
use std::net::TcpStream;
use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, TryRecvError};

/// Spawn a thread that reads user input (keyboard).
/// The spawning thread receives input to this thread via
/// the rx end of the channel.
pub fn spawn_stdin_channel() -> Receiver<String> {
    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();

        if buffer.starts_with("/") {
            process_command(&buffer);
        } else {
            // Send input across the channel, to the owning thread.
            tx.send(buffer).expect("Unable to transmit");
        }
    });

    rx
}

/// Read user (keyboard) input without blocking, and forward it
/// to the remote host.
pub fn read_user_input(stream: &mut TcpStream, receiver: &Receiver<String>) {
    match receiver.try_recv() {
        Ok(user_input) => {
            stream.write(&user_input.as_bytes()).expect("Unable to transmit");
        },
        Err(TryRecvError::Empty) => (), //println!("Error: Channel empty"),
        Err(TryRecvError::Disconnected) => (), //panic!("Error: Channel disconnected"),
    }
}

fn process_command(cmd: &str) {
    println!("process_command does nothing... {}", cmd);
}
