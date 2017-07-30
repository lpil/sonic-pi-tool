extern crate rosc;
extern crate nix;
extern crate ansi_term;

use std::process;
use std::path::Path;
use std::ffi::CString;
use nix::unistd::execv;

mod server;
mod file;
mod log_packet;

use std::io::{self, Read};

/// Read code from STDIN and send to Sonic Pi Server.
///
pub fn eval_stdin() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    server::run_code(input);
}


/// Read code from a file and send to Sonic Pi Server.
///
pub fn eval_file(path: &str) {
    match file::read(path) {
        Ok(code) => server::run_code(code),
        Err(msg) => {
            println!("{}", msg);
            process::exit(1);
        }
    }
}

/// Take some code and send to Sonic Pi Server.
///
pub fn eval(code: String) {
    server::run_code(code);
}


/// Check if something is listening on the Sonic Pi server's port.
/// If something is we can probably assume that it's the Sonic Pi Server,
/// so siginify this to the user.
///
pub fn check() {
    if server::server_port_in_use() {
        println!("Sonic Pi server listening on port 4557");
        process::exit(0);
    } else {
        println!("Sonic Pi server NOT listening on port 4557");
        process::exit(1);
    }
}


/// Instuct the Sonic Pi server to stop playing.
///
pub fn stop() {
    server::stop_all_jobs();
}


// TODO: Colour the word "error:"
const ADDR_IN_USE_MSG: &'static str =
    r#"error: Unable to listen for Sonic Pi server logs, address already in use.

This may because the Sonic Pi GUI is running and already listening on the desired port.
If the GUI is running this command cannot function, try running just the Sonic Pi server."#;

/// Print log messages sent by the Sonic Pi server.
/// This will fail if the GUI is running.
///
pub fn logs() {
    match server::follow_logs() {
        Err(server::FollowLogError::AddrInUse) => {
            println!("{}", ADDR_IN_USE_MSG);
            process::exit(1);
        }
        Err(server::FollowLogError::ReceiveFail(e)) => {
            println!("Unexpected error: {}\n", e);
            println!("Please report this error at https://github.com/lpil/sonic-pi-tool/issues");
            process::exit(1);
        }
        Ok(()) => (),
    };
}


/// Find the Sonic Pi server executable and run it. If it can be found.
///
pub fn start_server() {
    let paths = ["/Applications/Sonic Pi.app/server/bin/sonic-pi-server.rb",
                 "./app/server/bin/sonic-pi-server.rb",
                 "/opt/sonic-pi/app/server/bin/sonic-pi-server.rb",
                 "/usr/lib/sonic-pi/server/bin/sonic-pi-server.rb"];

    match paths.iter().find(|&&p| Path::new(p).exists()) {
        Some(p) => {
            execv(&CString::new(*p).unwrap(), &[]).expect(&format!("Unable to start {}", *p))
        }
        None => {
            println!("I couldn't find the Sonic Pi server executable :(");
            process::exit(1);
        }
    };
}
