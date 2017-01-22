use rosc::{OscPacket, OscType, OscMessage};

pub fn to_log_string(packet: OscPacket) -> String {
    match packet {
        OscPacket::Message(msg) => {
            println!("OSC address: {}", msg.addr);
            let log = match msg.addr.as_ref() {
                "/log/info" => format_log_info(msg),
                "/info" => format_log_info(msg),
                _ => None,
            };
            log.unwrap_or("".to_string())
        }
        OscPacket::Bundle(_bundle) => "".to_string(),
    }
}

fn format_log_info(msg: OscMessage) -> Option<String> {
    msg.args
        .as_ref()
        .and_then(|args| args.get(1))
        .and_then(|e| match e {
            &OscType::String(ref string) => Some(format!("=> \"{}\"\n", string)),
            _ => None,
        })
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
