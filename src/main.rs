mod util;
use util::gtinny;

use clap::{Arg, ArgAction, Command};
use std::process::exit;

fn main() {
    let matches = Command::new("gtinny")
        .version("0.1.4")
        .about("A GTIN Validator")
        .arg(Arg::new("gtin").help("Validates the GTIN"))
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Print if it is a valid GTIN")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .help("Suppress all logging. Overrides verbose behaviour.")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    if let Some(gtin) = matches.get_one::<String>("gtin") {
        if gtinny::is_valid(gtin) {
            if !matches.get_flag("quiet") && matches.get_flag("verbose") {
                println!("Valid GTIN: {}", gtin);
            }
            exit(0);
        } else {
            if !matches.get_flag("quiet") {
                println!("Invalid GTIN: {}", gtin);
            }
            exit(1);
        }
    }
}
