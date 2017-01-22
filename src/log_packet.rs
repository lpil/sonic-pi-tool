use rosc::OscPacket;

pub fn to_log_string(packet: OscPacket) -> String {
    match packet {
        OscPacket::Message(msg) => {
            println!("OSC address: {}", msg.addr);
            match msg.args {
                Some(args) => format!("OSC arguments: {:?}", args),
                None => format!("No arguments in message."),
            }
        }
        OscPacket::Bundle(bundle) => format!("OSC Bundle: {:?}", bundle),
    }
}
