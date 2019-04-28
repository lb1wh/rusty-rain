use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::time::Duration;
use std::fs;

use serde::{Serialize, Deserialize};

const READ_TIMEOUT: u64 = 10; // Seconds
const CONFIG: &str = "config.json";

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    hostname: String,
    port: u16,
}

fn main() -> Result<(), String> {
    println!("Rain 0.1.0");

    let config = fs::read_to_string(CONFIG);

    if let Err(e) = &config {
        return Err(format!("{} missing: {}", CONFIG, e));
    }

    let config: Result<Config, _> = serde_json::from_str(&config.unwrap());

    if let Err(e) = &config {
        return Err(format!("Unable to parse json in {}: {}", CONFIG, e));
    }

    let config = config.unwrap();

    let mut stream = TcpStream::connect(format!("{}:{}", &config.hostname, &config.port))
        .expect(&format!("Unable to connect to host: {}", &config.hostname));

    // Disable blocking for read, write, recv and send.
    stream.set_nonblocking(true).expect("Call to set_nonblocking failed");

    stream.set_read_timeout(Some(Duration::new(READ_TIMEOUT, 0)))
        .expect("Call to set_read_timeout failed");

    let mut network_in = vec![0; 80];

    // This approach doesn't assume that all inputs end in '\n' or EOF,
    // unlike BufReader and all the other read-functions.
    loop {
        match stream.read(&mut network_in) {
            Ok(_) => {
                for c in &network_in {
                    print!("{}", *c as char);
                }
                io::stdout().flush().unwrap();
                network_in = vec![0; 10];
            }
            // There was nothing to read. Are there specific
            // errors produced by read() that require special handling?
            Err(_) => {}
        };
    };
}
