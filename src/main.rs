// TODO: Enable GMCP-support (telnet option 201?)
use std::env;
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::time::Duration;
use std::fs;
use serde::{Serialize, Deserialize};

mod user_input;
mod telnet;

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
                // Received invalid UTF-8 - proceed to display
                // characters up until the invalid one.
                Err(e) => {
                    let invalid_pos = e.valid_up_to();
                    let (valid, _) = network_in.split_at(invalid_pos);
                    print!("{}", std::str::from_utf8(valid).unwrap());

                    // Validate and process telnet options
                    //telnet::process_opts(stream, network_in);
                }
            }

            io::stdout().flush().unwrap();
            for i in network_in.iter_mut() { *i = 0; }
        }
        // There was nothing to read. Are there specific
        // errors produced by read() that require special handling?
        Err(_) => {}
    };
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

    // Receive and process user input in a separate thread.
    let stdin_channel = user_input::spawn_stdin_channel();

    //let opt_parser = telnet::OptionParser::new();

    // This approach doesn't assume that all inputs end in '\n' or EOF,
    // unlike BufReader and several other reader-functions.
    loop {
        read_network(&mut stream, &mut network_in);
        user_input::read_user_input(&mut stream, &stdin_channel);
    };
}
