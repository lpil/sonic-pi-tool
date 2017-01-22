use rosc::{OscPacket, OscType, OscMessage};

pub fn to_log_string(packet: OscPacket) -> String {
    match packet {
        OscPacket::Message(msg) => {
            // println!("{:?}", msg);
            let log = match msg.addr.as_ref() {
                "/log/info" => format_log_info(msg),
                "/info" => format_log_info(msg),
                "/error" => format_error(msg),
                _ => None,
            };
            log.unwrap_or("".to_string())
        }
        OscPacket::Bundle(_bundle) => "".to_string(),
    }
}

fn format_log_info(msg: OscMessage) -> Option<String> {
    format_string_arg(msg, 1, |e| format!("=> {}\n", e))
}

fn format_error(msg: OscMessage) -> Option<String> {
    format_string_arg(msg, 1, |e| format!("Runtime Error: {}\n\n", e))
}


fn format_string_arg<F>(msg: OscMessage, index: usize, fmt: F) -> Option<String>
    where F: Fn(&String) -> String
{
    msg.args
        .as_ref()
        .and_then(|args| args.get(index))
        .and_then(|e| match e {
            &OscType::String(ref string) => Some(fmt(string)),
            _ => None,
        })
}


#[cfg(test)]
mod tests {
    use rosc::{OscPacket, OscMessage, OscType};
    use super::*;

    #[test]
    fn info_test() {
        let msg = OscPacket::Message(OscMessage {
            addr: "/info".to_string(),
            args: Some(vec![OscType::Int(1), OscType::String("Hello!".to_string())]),
        });
        assert_eq!("=> Hello!\n", to_log_string(msg));
    }

    #[test]
    fn log_info_test() {
        let msg = OscPacket::Message(OscMessage {
            addr: "/log/info".to_string(),
            args: Some(vec![OscType::Int(1), OscType::String("Hello!".to_string())]),
        });
        assert_eq!("=> Hello!\n", to_log_string(msg));
    }

    #[test]
    fn error_test() {
        let error_txt = r#"[]
Thread death +--&gt; :live_loop_no_sleep_loop
 Live loop :no_sleep_loop did not sleep!"#
            .to_string();
        let backtrace = r#"lang/thing.rb:3442:in `block in out_thread&#39;
lang/core.rb:2863:in `block in in_thread&#39;"#
            .to_string();
        let msg = OscPacket::Message(OscMessage {
            addr: "/error".to_string(),
            args: Some(vec![OscType::Int(24),
                            OscType::String(error_txt),
                            OscType::String(backtrace),
                            OscType::Int(1)]),
        });
        let expected = r#"Runtime Error: []
Thread death +--&gt; :live_loop_no_sleep_loop
 Live loop :no_sleep_loop did not sleep!

"#;
        assert_eq!(expected, to_log_string(msg));
    }
}
