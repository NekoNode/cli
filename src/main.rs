mod cli;
mod send;

const BANNER: &str = include_str!("statics/banner.txt");

fn main() {
    let args = cli::Cli::matches();

    if let Some(matches) = args.subcommand_matches("send") {
        if let Some(filename) = matches.value_of("Filename") {
            if let Err(err) = send::send_impl::send_file(filename) {
                eprintln!("{}", err);
            }
        } else if let Err(err) = send::send_impl::send_intro() {
            eprintln!("{}", err);
        }
    } else if let Some(matches) = args.subcommand_matches("receive") {
        send::receive_impl::receive_file(
            matches.value_of("Code").unwrap(),
            matches.value_of("Filename"),
        )
        .unwrap();
    } else {
        println!("{}", BANNER);
    }
}
