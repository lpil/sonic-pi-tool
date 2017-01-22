use std::net::UdpSocket;
use std::net;
use rosc;
use rosc::{encoder, OscPacket, OscMessage, OscType};
use super::log_packet;

type OscMsg = Vec<u8>;

pub enum FollowLogError {
    AddrInUse,
    ReceiveFail(String),
}


/// Check if something is listening on the Sonic Pi server's port.
///
pub fn server_port_in_use() -> bool {
    match UdpSocket::bind("127.0.0.1:4557") {
        Ok(_) => false,
        Err(_) => true,
    }
}

/// Takes a string of Sonic Pi source code and sends it to the Sonic Pi server.
///
pub fn run_code(source: String) {
    let client_name = OscType::String("SONIC_PI_TOOL".to_string());
    let osc_source = OscType::String(source);

    let msg = &OscPacket::Message(OscMessage {
        addr: "/run-code".to_string(),
        args: Some(vec![client_name, osc_source]),
    });
    let msg_buf = encoder::encode(msg).unwrap();
    send(msg_buf);
}


/// Instuct the Sonic Pi server to stop playing.
///
pub fn stop_all_jobs() {
    let client_name = OscType::String("SONIC_PI_TOOL".to_string());

    let msg = &OscPacket::Message(OscMessage {
        addr: "/stop-all-jobs".to_string(),
        args: Some(vec![client_name]),
    });
    let msg_buf = encoder::encode(msg).unwrap();
    send(msg_buf);
}

pub fn follow_logs() -> Result<(), FollowLogError> {
    let socket = match UdpSocket::bind("127.0.0.1:4558") {
        Err(_) => return Err(FollowLogError::AddrInUse),
        Ok(s) => s,
    };
    let mut buffer = [0u8; rosc::decoder::MTU];

    loop {
        match socket.recv_from(&mut buffer) {
            Ok((size, _addr)) => {
                let packet = rosc::decoder::decode(&buffer[..size]).unwrap();
                let log = log_packet::to_log_string(packet);
                print!("{}", log);
            }
            Err(e) => {
                return Err(FollowLogError::ReceiveFail(format!("{}", e)));
            }
        }
    }
}

/// Send an OSC message to the Sonic Pi server, which is presumed to be
/// listening on UDP port 4557.
///
/// We don't expect to recieve anything, so we bind to 0.0.0.0:0, which prompts
/// the OS to assign us an arbitrary unused port.
///
fn send(msg: OscMsg) {
    let localhost = net::Ipv4Addr::new(127, 0, 0, 1);
    let s_pi_addr = net::SocketAddrV4::new(localhost, 4557);

    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    socket.send_to(&msg, s_pi_addr).unwrap();
}
