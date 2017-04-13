use rosc::{OscPacket, OscType, OscMessage};
use ansi_term::Colour::{Black, Red, Blue, Purple, Yellow, Green};
use std::string::ToString;

pub fn to_log_string(packet: OscPacket) -> String {
    match packet {
        OscPacket::Message(msg) => {
            let log = match msg.addr.as_ref() {
                "/log/multi_message" |
                "/multi_message" => format_multi_message(msg),
                "/log/info" | "/info" => format_log_info(&msg),
                "/error" => format_error(&msg),
                "/syntax_error" => format_syntax_error(&msg),
                _ => None,
            };
            log.unwrap_or_else(String::new)
        }
        OscPacket::Bundle(_bundle) => String::new(),
    }
}

fn format_log_info(msg: &OscMessage) -> Option<String> {
    format_string_arg(msg, 1, |e| format!("=> {}\n", e))
}

fn format_error(msg: &OscMessage) -> Option<String> {
    format_string_arg(msg, 1, |e| format!("Runtime Error: {}\n\n", e))
}

fn format_syntax_error(msg: &OscMessage) -> Option<String> {
    format_string_arg(msg, 1, |e| format!("Syntax Error: {}\n\n", e))
}


fn format_string_arg<F>(msg: &OscMessage, index: usize, fmt: F) -> Option<String>
    where F: Fn(&String) -> String
{
    msg.args
        .as_ref()
        .and_then(|args| args.get(index))
        .and_then(|e| match *e {
                      OscType::String(ref string) => Some(fmt(string)),
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

impl Message {
    pub fn new(msg_type: &OscType, info: &OscType) -> Option<Message> {
        match (msg_type, info) {
            (&OscType::Int(i), &OscType::String(ref s)) => {
                Some(Message {
                         msg_type: i,
                         info: s.to_string(),
                     })
            }
            _ => None,
        }
    }

    // msg_type to colour according to Sonic Pi GUI
    // 0:     fg deeppink
    // 1:     fg dodgerblue
    // 2:     fg darkorange
    // 3:     fg red
    // 4:     fg white      bg deeppink
    // 5:     fg white      bg dodgerblue
    // 6:     fg white      bg darkorange
    // other: fg green
    pub fn write_str(&self, buffer: &mut String) {
        match self.msg_type {
            0 => buffer.push_str(&format!("{}", Purple.paint(self.info.clone()))),
            1 => buffer.push_str(&format!("{}", Blue.paint(self.info.clone()))),
            2 => buffer.push_str(&format!("{}", Yellow.paint(self.info.clone()))),
            3 => buffer.push_str(&format!("{}", Red.paint(self.info.clone()))),
            4 => buffer.push_str(&format!("{}", Black.on(Purple).paint(self.info.clone()))),
            5 => buffer.push_str(&format!("{}", Black.on(Blue).paint(self.info.clone()))),
            6 => buffer.push_str(&format!("{}", Black.on(Yellow).paint(self.info.clone()))),
            _ => buffer.push_str(&format!("{}", Green.paint(self.info.clone()))),
        }
    }
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
        let mut args = match msg.args {
            Some(a) => a.into_iter(),
            _ => return None,
        };
        let (job_id, thread_name, runtime, num_msgs) =
            match (args.next(), args.next(), args.next(), args.next()) {
                (Some(OscType::Int(job)),
                 Some(OscType::String(thread)),
                 Some(OscType::String(runtime)),
                 Some(OscType::Int(num_msgs))) => (job, thread, runtime, num_msgs),
                _ => return None,
            };

        let mut messages = Vec::with_capacity(num_msgs as usize);
        while let (Some(msg_type), Some(info)) = (args.next(), args.next()) {
            if let Some(msg) = Message::new(&msg_type, &info) {
                messages.push(msg);
            }
        }

        let multi = MultiMessage {
            job_id: job_id,
            thread_name: thread_name.to_string(),
            runtime: runtime.to_string(),
            messages: messages,
        };
        Some(multi)
    }

    pub fn format(&self) -> String {
        let mut buffer = String::new();
        buffer.push_str(&format!("[Run {}, Time {}]", self.job_id, self.runtime));

        match self.messages.len() {
            0 => (),
            1 => {
                buffer.push_str("\n └ ");
                self.messages[0].write_str(&mut buffer);
            }
            n => {
                for i in 0..(n - 1) {
                    buffer.push_str("\n ├ ");
                    self.messages[i].write_str(&mut buffer);
                }
                buffer.push_str("\n └ ");
                self.messages[n - 1].write_str(&mut buffer);
            }
        }
        buffer.push_str("\n");
        buffer
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
        let expected = "[Run 2, Time 1293.1]\n".to_string();
        let output = to_log_string(msg);
        println!("expected:{}", expected);
        println!("actual:{}", output);
        assert_eq!(expected, output);
    }

    #[test]
    fn log_multi_message_no_msgs_test() {
        let job_id = OscType::Int(2);
        let thread_name = OscType::String("name".to_string());
        let runtime = OscType::String("1293.1".to_string());
        let num_msgs = OscType::Int(0);
        let msg = OscPacket::Message(OscMessage {
                                         addr: "/log/multi_message".to_string(),
                                         args: Some(vec![job_id, thread_name, runtime, num_msgs]),
                                     });
        let expected = "[Run 2, Time 1293.1]\n".to_string();
        let output = to_log_string(msg);
        println!("expected:{}", expected);
        println!("actual:{}", output);
        assert_eq!(expected, output);
    }

    #[test]
    fn multi_message_one_msgs_test() {
        let job_id = OscType::Int(2);
        let thread_name = OscType::String("name".to_string());
        let runtime = OscType::String("1293.1".to_string());
        let num_msgs = OscType::Int(1);
        let msg1_type = OscType::Int(0);
        let msg1_info = OscType::String("synth :beep".to_string());
        let msg = OscPacket::Message(OscMessage {
                                         addr: "/multi_message".to_string(),
                                         args: Some(vec![job_id,
                                                         thread_name,
                                                         runtime,
                                                         num_msgs,
                                                         msg1_type,
                                                         msg1_info]),
                                     });
        let expected = format!(r#"[Run 2, Time 1293.1]
 └ {}
"#,
                               Purple.paint("synth :beep"))
                .to_string();
        let output = to_log_string(msg);
        println!("expected:{}", expected);
        println!("actual:{}", output);
        assert_eq!(expected, output);
    }

    #[test]
    fn multi_message_test() {
        let job_id = OscType::Int(2);
        let thread_name = OscType::String("name".to_string());
        let runtime = OscType::String("1293.1".to_string());
        let num_msgs = OscType::Int(2);
        let msg1_type = OscType::Int(0);
        let msg1_info = OscType::String("synth :beep".to_string());
        let msg2_type = OscType::Int(11);
        let msg2_info = OscType::String("synth :boop".to_string());
        let msg = OscPacket::Message(OscMessage {
                                         addr: "/multi_message".to_string(),
                                         args: Some(vec![job_id,
                                                         thread_name,
                                                         runtime,
                                                         num_msgs,
                                                         msg1_type,
                                                         msg1_info,
                                                         msg2_type,
                                                         msg2_info]),
                                     });
        let expected = format!(r#"[Run 2, Time 1293.1]
 ├ {}
 └ {}
"#,
                               Purple.paint("synth :beep"),
                               Green.paint("synth :boop"))
                .to_string();
        let output = to_log_string(msg);
        println!("expected:{}", expected);
        println!("actual:{}", output);
        assert_eq!(expected, output);
    }

    #[test]
    fn info_test() {
        let msg = OscPacket::Message(OscMessage {
                                         addr: "/info".to_string(),
                                         args: Some(vec![OscType::Int(1),
                                                         OscType::String("Hello!".to_string())]),
                                     });
        assert_eq!("=> Hello!\n", to_log_string(msg));
    }

    #[test]
    fn log_info_test() {
        let msg = OscPacket::Message(OscMessage {
                                         addr: "/log/info".to_string(),
                                         args: Some(vec![OscType::Int(1),
                                                         OscType::String("Hello!".to_string())]),
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
                                         args: Some(vec![OscType::Int(24),
                                                         OscType::String(error_txt),
                                                         OscType::Int(1)]),
                                     });
        let expected = "Syntax Error: a.rb:1: syntax error, unexpected end-of-input\n\n"
            .to_string();
        assert_eq!(expected, to_log_string(msg));
    }
}
