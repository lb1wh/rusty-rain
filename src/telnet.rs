// Telnet option handling - mainly for GMCP-support.
//
// The general process for advanced telnet options involves subnegotiation.
// This means that the server and client first agree to communicate an
// option, and then proceed with subnegotiation for said option.
//
// Example:
// server: IAC DO GMCP
// client: IAC WILL GMCP
// client: IAC SB GMCP '...' IAC SE
// server: IAC SB GMCP '...' IAC SE
use std::net::TcpStream;
use std::io::Write;

struct Command {
    name: &'static str,
    code: u8,
}

/// See RFC854: TELNET PROTOCOL SPECIFICATION.
const GMCP: Command = Command { name: "GMCP", code: 201 };
const SE: Command = Command { name: "SE", code: 240 };
const SB: Command = Command { name: "SB", code: 250 };
const WILL: Command = Command { name: "WILL", code: 251 };
const WONT: Command = Command { name: "WONT", code: 252};
const DO: Command = Command { name: "DO", code: 253 };
const DONT: Command = Command { name: "DONT", code: 254 };
const IAC: Command = Command { name: "IAC", code: 255 };

fn debug_options(code: u8) {
    if code == GMCP.code {
        print!("{} ", GMCP.name);
    } else if code == SE.code {
        print!("{} ", SE.name);
    } else if code == SB.code {
        print!("{} ", SB.name);
    } else if code == WILL.code {
        print!("{} ", WILL.name);
    } else if code == WONT.code{
        print!("{} ", WONT.name);
    } else if code == DO.code{
        print!("{} ", DO.name);
    } else if code == DONT.code{
        print!("{} ", DONT.name);
    } else if code == IAC.code{
        print!("{} ", IAC.name);
    } else {
        print!("{} ", code);
    }
}

enum ProcessingStage {
    Inactive,
    Active,

}

/// Process telnet options.
pub fn process_opts(stream: &mut TcpStream, command: &[u8]) {
    let mut state = ProcessingStage::Inactive;

    for &code in command {
        //debug_options(code);

        // Interpret As Command (IAC), i.e. what comes next
        // is a telnet code.
        match state {
            ProcessingStage::Inactive => {
                if code == 255 { // IAC
                    print!("IAC ");
                    state = ProcessingStage::Active;
                }
            }
            ProcessingStage::Active => {
                if code == 0 {}
            }
        }
    }
}

// Make sure that we're looking at a valid telnet option.
//pub fn valid_option(command: &[u8]) -> bool {
//    print!("***{:?}***", command);
//    if command.len() >= 2 {
//        if command[0] == telnet_sequence::IAC {
//            print!("[{},{}", command[0], command[1]); // tmp
//            if command.len() > 2 { print!(",{}]", command[2]); } else { print!("]"); } // tmp
//            if command[1] == telnet_sequence::DO || command[1] == telnet_sequence::WILL {
//                return true;
//            }
//        }
//    }
//    false
//}

// Process telnet options.
// See RFC854: TELNET PROTOCOL SPECIFICATION.
//pub fn process(stream: &mut TcpStream, command: &[u8]) {
//    for byte in command {
//        match byte {
//            &telnet_sequence::iac.code => print!("IAC "),
//            &telnet_sequence::DO => print!("DO "),
//            _ => print!("? "),
//        }
        //print!("{},", byte);
//    }

//    if command.len() != 3 {
//        return ();
//    }
//
//    if command[2] == telnet_sequence::GMCP {
//        print!("Received telnet option sequence - and it was GMCP!"); // tmp
//        print!("Attempting to send reply for enabling it..."); // tmp
//        let option = vec![telnet_sequence::IAC,
//                          telnet_sequence::DO,
//                          telnet_sequence::GMCP];
//        //let option: String = option.iter().map(|&x| x as char).collect();
//        //let option: &[u8] = option.iter().map(|&x| x as char).collect();
//        //let option: &[u8] = option.iter().map(|&x| x as char);
//
//        stream.write(&option.as_ref()).expect("Unable to transmit");
//    }
//}
