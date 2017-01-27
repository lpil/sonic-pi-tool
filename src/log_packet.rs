use rosc::{OscPacket, OscType, OscMessage};

pub fn to_log_string(packet: OscPacket) -> String {
    match packet {
        OscPacket::Message(msg) => {
            // println!("{:?}", msg);
            let log = match msg.addr.as_ref() {
                "/log/info" => format_log_info(msg),
                "/info" => format_log_info(msg),
                "/error" => format_error(msg),
                "/syntax_error" => format_syntax_error(msg),
                "/multi_message" => format_multi_message(msg),
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

fn format_syntax_error(msg: OscMessage) -> Option<String> {
    format_string_arg(msg, 1, |e| format!("Syntax Error: {}\n\n", e))
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

fn format_multi_message(msg: OscMessage) -> Option<String> {
    MultiMessage::new(msg).map(|m| m.format())
}

#[derive(Debug)]
struct Message {
    msg_type: i32,
    info: String,
}

#[derive(Debug)]
struct MultiMessage {
    job_id: i32,
    thread_name: String,
    runtime: String,
    messages: Vec<Message>,
}

impl MultiMessage {
    pub fn new(msg: OscMessage) -> Option<MultiMessage> {
        let args = match msg.args {
            Some(a) => a,
            _ => return None,
        };
        let job_id = match args.get(0) {
            Some(&OscType::Int(i)) => i,
            _ => return None,
        };
        let thread_name = match args.get(1) {
            Some(&OscType::String(ref s)) => s,
            _ => return None,
        };
        let runtime = match args.get(2) {
            Some(&OscType::String(ref s)) => s,
            _ => return None,
        };
        let num_msgs = match args.get(3) {
            Some(&OscType::Int(i)) => i,
            _ => return None,
        };
        let multi = MultiMessage {
            job_id: job_id,
            thread_name: thread_name.to_string(),
            runtime: runtime.to_string(),
            messages: vec![],
        };
        Some(multi)
    }

    pub fn format(&self) -> String {
        format!("\n[Run {}, Time {}]", self.job_id, self.runtime)
    }
}

#[cfg(test)]
mod tests {
    use rosc::{OscPacket, OscMessage, OscType};
    use super::*;

    #[test]
    fn multi_message_no_msgs_test() {
        let job_id = OscType::Int(2);
        let thread_name = OscType::String("name".to_string());
        let runtime = OscType::String("1293.1".to_string());
        let num_msgs = OscType::Int(0);
        let msg = OscPacket::Message(OscMessage {
            addr: "/multi_message".to_string(),
            args: Some(vec![job_id, thread_name, runtime, num_msgs]),
        });
        let expected = "\n[Run 2, Time 1293.1]".to_string();
        assert_eq!(expected, to_log_string(msg));
    }

    //     #[test]
    //     fn multi_message_test() {
    //         let job_id = OscType::Int(2);
    //         let thread_name = OscType::String("name".to_string());
    //         let runtime = OscType::String("1293.1".to_string());
    //         let num_msgs = OscType::Int(2);
    //         let msg1_type = OscType::Int(0);
    //         let msg1_info = OscType::String("synth :beep".to_string());
    //         let msg2_type = OscType::Int(1);
    //         let msg2_info = OscType::String("synth :boop".to_string());
    //         let msg = OscPacket::Message(OscMessage {
    //             addr: "/multi_message".to_string(),
    //             args: Some(vec![job_id,
    //                             thread_name,
    //                             runtime,
    //                             num_msgs,
    //                             msg1_type,
    //                             msg1_info,
    //                             msg2_type,
    //                             msg2_info]),
    //         });
    //         let expected = r#"\n[Run 2, Time 1293.1]
    //  ├ synth :beep
    //  └ synth :boop
    // "#
    //             .to_string();
    //         assert_eq!(expected, to_log_string(msg));
    //     }

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

    #[test]
    fn syntax_error_test() {
        let error_txt = "a.rb:1: syntax error, unexpected end-of-input".to_string();
        let msg = OscPacket::Message(OscMessage {
            addr: "/syntax_error".to_string(),
            args: Some(vec![OscType::Int(24), OscType::String(error_txt), OscType::Int(1)]),
        });
        let expected = "Syntax Error: a.rb:1: syntax error, unexpected end-of-input\n\n"
            .to_string();
        assert_eq!(expected, to_log_string(msg));
    }
}
