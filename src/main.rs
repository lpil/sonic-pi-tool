#[macro_use]
extern crate clap;
use clap::{App, AppSettings, Arg, SubCommand};

extern crate lib;

fn main() {
    let cli_app = App::new("sonic-pi-tool")
        .author("Louis Pilfold <louis@lpil.uk>")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::ColoredHelp)
        .version(crate_version!());

    let check = SubCommand::with_name("check").about(
        "Check if the Sonic Pi server is listening on port 4557",
    );

    let eval = SubCommand::with_name("eval")
        .about("Takes a string of Sonic Pi code and sends it to the server")
        .arg(
            Arg::with_name("CODE")
                .help("A string of Sonic Pi code")
                .required(true)
                .index(1),
        );

    let eval_stdin = SubCommand::with_name("eval-stdin").about(
        "Reads Sonic Pi code from stdin and sends it to the server",
    );

    let eval_file = SubCommand::with_name("eval-file")
        .about("Reads Sonic Pi code from a file and sends it to the server")
        .arg(
            Arg::with_name("PATH")
                .help("Path to the file of Sonic Pi code")
                .required(true)
                .index(1),
        );

    let start_server =
        SubCommand::with_name("start-server").about("Find and start the Sonic Pi server");

    let stop =
        SubCommand::with_name("stop").about("Stops all currently playing music on the server");

    let logs =
        SubCommand::with_name("logs").about("Print log events emitted by the Sonic Pi server");

    let record = SubCommand::with_name("record")
        .about("Record the audio output of a Sonic Pi session")
        .arg(
            Arg::with_name("PATH")
                .help("Absolute path to the output file")
                .required(true)
                .index(1),
        );

    let matches = cli_app
        .subcommand(stop)
        .subcommand(check)
        .subcommand(logs)
        .subcommand(eval)
        .subcommand(eval_stdin)
        .subcommand(eval_file)
        .subcommand(start_server)
        .subcommand(record)
        .get_matches();

    match matches.subcommand_name() {
        Some("stop") => lib::stop(),
        Some("check") => lib::check(),
        Some("eval") => do_eval(&matches),
        Some("eval-file") => do_eval_file(&matches),
        Some("eval-stdin") => lib::eval_stdin(),
        Some("start-server") => lib::start_server(),
        Some("logs") => lib::logs(),
        Some("record") => do_record(&matches),
        _ => panic!("This should be unreachable."),
    }
}

fn do_eval_file(matches: &clap::ArgMatches) {
    let path = matches
        .subcommand_matches("eval-file")
        .unwrap()
        .value_of("PATH")
        .unwrap()
        .to_string();
    lib::eval_file(&path);
}

fn do_eval(matches: &clap::ArgMatches) {
    let code = matches
        .subcommand_matches("eval")
        .unwrap()
        .value_of("CODE")
        .unwrap()
        .to_string();
    lib::eval(code);
}

fn do_record(matches: &clap::ArgMatches) {
    let path = matches
        .subcommand_matches("record")
        .unwrap()
        .value_of("PATH")
        .unwrap()
        .to_string();
    lib::record(&path);
}
