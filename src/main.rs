mod util;
use util::gtinny;

use clap::{Arg, Command, ArgAction};
use std::process::exit;

fn main() {
    let matches = Command::new("gtinny")
        .version("0.1.0")
        .about("A GTIN Validator")
        .arg(Arg::new("gtin").help("Validates the GTIN"))
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Print if it is a valid GTIN")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    if let Some(gtin) = matches.get_one::<String>("gtin") {
        if gtinny::is_valid(gtin) {
            if matches.get_flag("verbose") {
                println!("Valid GTIN: {}", gtin);
            }
            exit(0);
        } else {
            println!("Invalid GTIN: {}", gtin);
            exit(1);
        }
    }
}
