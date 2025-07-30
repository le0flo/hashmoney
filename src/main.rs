use std::process::exit;

use clap::{arg, value_parser, Command};
use hashmoney::{MintStrategy, Stamp};

fn cli() -> Command {
    return Command::new("Hashmoney")
        .bin_name("hm")
        .about("Yet another hashcash implementation")
        .subcommand(Command::new("mint")
            .about("Mints a new stamp")
            .args(vec![
                arg!(-b --bits <bits> "The bits of the stamp")
                    .value_parser(value_parser!(u32)),
                arg!(-w --date_width [date_width] "The date field width (can only be: 6, 10 or 12)")
                    .value_parser(value_parser!(usize)),
                arg!(<resource> "The resource of the stamp")
                    .value_parser(value_parser!(String)),
            ])
        )
        .subcommand(Command::new("check")
            .about("Checks a given stamp")
            .args(vec![
                arg!(-b --bits <bits> "The expected number of bits")
                    .value_parser(value_parser!(u32)),
                arg!(-d --days <days> "The number of validity days")
                    .value_parser(value_parser!(u32)),
                arg!(-r --resource <resource> "The expected resource")
                    .value_parser(value_parser!(String)),
                arg!(<stamp> "The stamp to check")
                    .value_parser(value_parser!(String)),
            ])
        );
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("mint", sub_matches)) => {
            let bits = sub_matches.get_one::<u32>("bits")
                .expect("A bit number is required")
                .clone();

            let date_width = sub_matches.get_one::<usize>("date_width")
                .unwrap_or(&6)
                .clone();

            let resource = sub_matches.get_one::<String>("resource")
                .expect("A resource is required")
                .clone();

            let stamp = Stamp::mint(bits, date_width, &resource, MintStrategy::Naive);
            println!("hashcash stamp: {}", stamp.to_string());
        },
        Some(("check", sub_matches)) => {
            let bits = sub_matches.get_one::<u32>("bits")
                .expect("Bits is a required argument")
                .clone();

            let days = sub_matches.get_one::<u32>("days")
                .expect("Days is a required argument")
                .clone();

            let resource = sub_matches.get_one::<String>("resource")
                .expect("Resource is a required argument")
                .clone();

            let stamp_str = sub_matches.get_one::<String>("stamp")
                .expect("A stamp is required")
                .clone();

            let stamp = Stamp::try_from(stamp_str).unwrap_or_else(|err| {
               println!("An error occured while parsing the given stamp: {}", err);
               exit(-2);
            });

            let result = stamp.check(bits, days, &resource);
            println!("Is {} valid?: {}", stamp.to_string(), if result.is_ok() { "yes".to_string() } else { result.unwrap_err().to_string() });
        },
        _ => {
            println!("Invalid subcommand");
            exit(-1)
        },
    };
}
