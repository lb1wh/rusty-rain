// TODO: Enable GMCP-support (telnet option 201?)
use std::env;
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::time::Duration;
use std::fs;
use serde::{Serialize, Deserialize};

mod user_input_thread;

#[derive(Serialize, Deserialize, Debug)]
struct Target {
    hostname: String,
    port: u16,
}

#[derive(Serialize, Deserialize, Debug)]
struct Network {
    input_buflen: usize,
    read_timeout: u64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    target: Target,
    network: Network,
}

fn read_config() -> Config {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Please specify a configuration file");
    }

    let config = fs::read_to_string(&args[1]).expect("Unable to read configuration file");
    let config = serde_yaml::from_str(&config)
        .expect("Unable to parse configuration file");

    config
}

/// Receive input from the remote host.
fn read_network(stream: &mut TcpStream, network_in: &mut Vec<u8>) {
    match stream.read(network_in) {
        Ok(_) => {
            match std::str::from_utf8(network_in) {
                Ok(data) => print!("{}", data),
                Err(e) => print!("[{}]", e),
            }

            io::stdout().flush().unwrap();
            for i in network_in.iter_mut() { *i = 0; }
        }
        // There was nothing to read. Are there specific
        // errors produced by read() that require special handling?
        Err(_) => {}
    };
}

/// Enable the Generic MUD Communication Protocol, such that
/// we receive heartbeats with metadata from the server.
fn enable_gmcp(stream: &mut TcpStream) {
    let tn_iac = 255u8 as char;   // Telnet IAC (Interpret As Command)
    let tn_do = 253u8 as char;    // Telnet option code DO
    let opt_gmcp = 201u8 as char; // Option Generic MUD Communication Protocol

    let option = vec![tn_iac, tn_do, opt_gmcp];
    let option: String = option.into_iter().collect();

    stream.write(&option.as_ref()).expect("Unable to transmit");

    // Is any subnegotiation required here? I.e. a sequence of
    // TN_SB, options, TN_SE for the GMCP-option, where SB denotes the start of
    // subnegotiation and TN_SE denotes the end of subnegotiation.
}

fn main() {
    println!("Rain 0.1.0");

    let config = read_config();

    let mut stream = TcpStream::connect(
        format!("{}:{}", &config.target.hostname, &config.target.port))
        .expect(&format!("Unable to connect to host: {}", &config.target.hostname));

    // Disable blocking for read, write, recv and send.
    stream.set_nonblocking(true).expect("Call to set_nonblocking failed");

    stream.set_read_timeout(Some(Duration::new(config.network.read_timeout, 0)))
        .expect("Call to set_read_timeout failed");

    let mut network_in = vec![0; config.network.input_buflen];
    // Receive user input in a separate thread.
    let stdin_channel = user_input_thread::spawn_stdin_channel();

    enable_gmcp(&mut stream);

    // This approach doesn't assume that all inputs end in '\n' or EOF,
    // unlike BufReader and several other reader-functions.
    loop {
        read_network(&mut stream, &mut network_in);
        user_input_thread::read_user_input(&mut stream, &stdin_channel);
    };
}
