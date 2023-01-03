extern crate ansi_term;
extern crate nix;
extern crate rosc;
extern crate dirs;
extern crate duct;
extern crate toml;

use std::{thread, time};
use std::io::{self, Read};
use std::path::Path;
use std::process;

use duct::cmd;

mod file;
mod log_packet;
mod server;
mod config;

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
    let cfg = config::SonicPiToolCfg::read_from_path(config::SonicPiToolCfg::get_default_cfg_file_path());
    if server::server_port_in_use(cfg.sonic_pi_port) {
        println!("Sonic Pi server listening on port {}", cfg.sonic_pi_port);
        process::exit(0);
    } else {
        println!("Sonic Pi server NOT listening on port {}", cfg.sonic_pi_port);
        process::exit(1);
    }
}

/// Instuct the Sonic Pi server to stop playing.
///
pub fn stop() {
    server::stop_all_jobs();
}

// TODO: Colour the word "error:"
const ADDR_IN_USE_MSG: &str =
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
    let mut paths = vec![
        String::from("/Applications/Sonic Pi.app/Contents/Resources/app/server/ruby/bin"),
        String::from("/Applications/Sonic Pi.app/server/bin"),
        String::from("/Applications/Sonic Pi.app/server/ruby/bin"),
        String::from("./app/server/bin"),
        String::from("/opt/spider/app/server/bin"),
        String::from("/usr/lib/spider/server/bin"),
        String::from("/opt/spider/app/server/ruby/bin"),
        String::from("/usr/lib/spider/server/ruby/bin"),
    ];

    if let Some(home_directory) = dirs::home_dir() {
        let suffix = "Applications/Sonic Pi.app/server/bin";
        let home = format!("{}/{}", home_directory.to_str().unwrap(), suffix);

        paths.insert(0, home);
    };

    match paths
        .iter()
        .find(|p| {
            Path::new(&p).exists()
        }) {
        Some(p) => {
            let daemon_exe = format!("{}/daemon.rb", p);

            let mut reader = cmd!(daemon_exe, "--no-scsynth-inputs").reader().unwrap();
            let mut buff: [u8; 1000] = [0; 1000];
            let read_bytes = reader.read(&mut buff).unwrap();
            let output = String::from_utf8_lossy(&buff[0..read_bytes]);
            println!("Daemon Output: {}", output);
            let out_values:Vec<&str> = output.split(' ')
                                    .map(|s| s.trim())
                                    .collect();

            /* Anatomy of the daemon output
             *
             * daemon.rb outputs:
             * 39097 39099 39098 39100 39101 39102 39104 2070055865
             * |     |     |                             |
             * +-----]---> Port on which the Daemon listens. Needed for Keep-Alive messages 
             *       |     |                             |
             *       |     |                             |
             *       |     |                             |
             *       +--> Gui Port - For the "logs" command 
             *             |                             |
             *             |                             |
             *             +--> Server Port - Send Commands to this port
             *                                           |
             *                                           |
             *                                           +---> Token required to communicate with
             *                                           the other processes
             * */
            let daemon_port = out_values[0].parse::<u16>().unwrap();
            let gui_port = out_values[1].parse::<u16>().unwrap();
            let sonic_pi_port = out_values[2].parse::<u16>().unwrap();
            let token = out_values[7].parse::<i32>().unwrap();

            // Write ~/.sonic-pi/tool/ports.toml
            std::fs::create_dir_all(&config::SonicPiToolCfg::get_default_cfg_folder()).unwrap();
            let cur_cfg = &config::SonicPiToolCfg::new(out_values, token, sonic_pi_port, daemon_port, gui_port);
            std::fs::write(&config::SonicPiToolCfg::get_default_cfg_file_path(),
                           toml::to_string(cur_cfg).unwrap()).unwrap();

            loop {
                server::send_keep_live();
                thread::sleep(time::Duration::from_secs(5));
            };
        }
        None => {
            println!("I couldn't find the Sonic Pi server executable :(");
            process::exit(1);
        }
    };
}

/// Record the audio output of a Sonic Pi session to a local file.
/// Stop and save the recording when the <Enter> key is pressed.
///
pub fn record(path: &str) {
    server::start_recording();
    println!("Recording started, saving to {}", path);
    println!("Press Enter to stop the recording...");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            server::stop_and_save_recording(path.to_string());
        }
        Err(error) => {
            println!("error: {}", error);
            server::stop_and_save_recording(path.to_string());
        }
    }
}
