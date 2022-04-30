use clap::{arg, command, ArgMatches, Command};

pub struct Cli {}

impl Cli {
    pub fn matches() -> ArgMatches {
        let matches = command!()
            .subcommand(
                Command::new("send")
                    .about("Share file to friends")
                    .arg(arg!([Filename] "File to share, empty string for introduction")),
            )
            .subcommand(
                Command::new("receive")
                    .about("Receive file")
                    .arg(arg!(<Code> "Share code"))
                    .arg(arg!([Filename] "Output file")),
            )
            .get_matches();

        matches
    }
}
