use rosc::{OscPacket, OscType, OscMessage};

pub fn to_log_string(packet: OscPacket) -> String {
    match packet {
        OscPacket::Message(msg) => {
            println!("OSC address: {}", msg.addr);
            match msg.addr.as_ref() {
                "/log/info" => format_log_info(msg),
                "/info" => format_log_info(msg),
                _ => "".to_string(),
            }
        }
        OscPacket::Bundle(bundle) => format!("Expected message {:?}", bundle),
    }
}

fn format_log_info(msg: OscMessage) -> String {
    let args = msg.args.unwrap();
    if let &OscType::String(ref string) = args.get(1).unwrap() {
        format!("=> \"{}\"\n", string)
    } else {
        "".to_string()
    }
}



#[cfg(test)]
mod tests {
    use rosc::{OscPacket, OscMessage, OscType};
    use super::*;

    #[test]
    fn log_info_test() {
        let msg = OscPacket::Message(OscMessage {
            addr: "/info".to_string(),
            args: Some(vec![OscType::Int(1), OscType::String("Hello!".to_string())]),
        });
        assert_eq!("=> \"Hello!\"\n", to_log_string(msg));
    }
}
