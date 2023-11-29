use super::log_packet;
use rosc;
use rosc::{encoder, OscMessage, OscPacket, OscType};
use std::net;
use std::net::UdpSocket;

#[path="config.rs"]
mod config;
use config::SonicPiToolCfg;

pub enum FollowLogError {
    AddrInUse,
    ReceiveFail(String),
}

/// Check if something is listening on the Sonic Pi server's port.
///
pub fn server_port_in_use(port: u16) -> bool {
    UdpSocket::bind(format!("127.0.0.1:{}", port)).is_err()
}

/// Takes a string of Sonic Pi source code and sends it to the Sonic Pi server.
///
pub fn run_code(source: String) {
    let cfg = SonicPiToolCfg::read_from_path(SonicPiToolCfg::get_default_cfg_file_path());
    let token = OscType::Int(cfg.token);
    let osc_source = OscType::String(source);

    let msg = &OscPacket::Message(OscMessage {
        addr: "/run-code".to_string(),
        args: Some(vec![token, osc_source]),
    });
    let msg_buf = encoder::encode(msg).unwrap();
    send(&msg_buf, cfg.sonic_pi_port);
}

/// Instuct the Sonic Pi server to stop playing.
///
pub fn stop_all_jobs() {
    let cfg = SonicPiToolCfg::read_from_path(SonicPiToolCfg::get_default_cfg_file_path());
    let token = OscType::Int(cfg.token);

    let msg = &OscPacket::Message(OscMessage {
        addr: "/stop-all-jobs".to_string(),
        args: Some(vec![token]),
    });
    let msg_buf = encoder::encode(msg).unwrap();
    send(&msg_buf, cfg.sonic_pi_port);
}

pub fn follow_logs() -> Result<(), FollowLogError> {
    let cfg = SonicPiToolCfg::read_from_path(SonicPiToolCfg::get_default_cfg_file_path());
    let socket = match UdpSocket::bind(format!("127.0.0.1:{}", cfg.gui_port)) {
        Err(e) => {
            println!("{:?}", e);
            return Err(FollowLogError::AddrInUse)
        },
        Ok(s) => s,
    };
    let mut buffer = [0u8; rosc::decoder::MTU];

    loop {
        match socket.recv_from(&mut buffer) {
            Ok((size, _addr)) => {
                let packet = rosc::decoder::decode(&buffer[..size]).unwrap();
                let log = log_packet::to_log_string(packet);
                println!("{}", log);
            }
            Err(e) => {
                return Err(FollowLogError::ReceiveFail(format!("{}", e)));
            }
        }
    }
}

pub fn start_recording() {
    let cfg = SonicPiToolCfg::read_from_path(SonicPiToolCfg::get_default_cfg_file_path());
    let token = OscType::Int(cfg.token);

    let msg = &OscPacket::Message(OscMessage {
        addr: "/start-recording".to_string(),
        args: Some(vec![token]),
    });
    let msg_buf = encoder::encode(msg).unwrap();
    send(&msg_buf, cfg.sonic_pi_port);
}

pub fn stop_and_save_recording(path: String) {
    let cfg = SonicPiToolCfg::read_from_path(SonicPiToolCfg::get_default_cfg_file_path());
    let token = OscType::Int(cfg.token);
    let stop = &OscPacket::Message(OscMessage {
        addr: "/stop-recording".to_string(),
        args: Some(vec![token.clone()]),
    });
    let stop_buf = encoder::encode(stop).unwrap();
    send(&stop_buf, cfg.sonic_pi_port);

    let output_file = OscType::String(path);
    let save = &OscPacket::Message(OscMessage {
        addr: "/save-recording".to_string(),
        args: Some(vec![
            token,
            output_file,
        ]),
    });
    let save_buf = encoder::encode(save).unwrap();
    send(&save_buf, cfg.sonic_pi_port);
}

pub fn send_keep_live() {
    let cfg = SonicPiToolCfg::read_from_path(SonicPiToolCfg::get_default_cfg_file_path());
    let msg = &rosc::OscPacket::Message(rosc::OscMessage {
        addr: "/daemon/keep-alive".to_string(),
        args: Some(vec![rosc::OscType::Int(cfg.token)]),
    });
    let msg_buf = encoder::encode(msg).unwrap();
    send(&msg_buf, cfg.daemon_port);
}

/// Send an OSC message to the Server listening on the specified port 
/// We don't expect to recieve anything, so we bind to 0.0.0.0:0, which prompts
/// the OS to assign us an arbitrary unused port.
///
fn send(msg: &[u8], port: u16) {
    let localhost = net::Ipv4Addr::new(127, 0, 0, 1);
    let s_pi_addr = net::SocketAddrV4::new(localhost, port);

    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    socket.send_to(msg, s_pi_addr).unwrap();
}
