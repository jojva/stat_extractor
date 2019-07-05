extern crate clap;

use clap::{App, Arg, ArgMatches, SubCommand};

struct Top {
    input_file: String,
    nb_top_queries: u32,
    from: Option<u32>,
    to: Option<u32>,
}

struct Distinct {
    input_file: String,
    from: Option<u32>,
    to: Option<u32>,
}

impl Top {
    fn new(matches: &ArgMatches) -> Top {
        Top {
            input_file: matches.value_of("input_file").unwrap().to_owned(),
            nb_top_queries: matches
                .value_of("nb_top_queries")
                .unwrap()
                .parse::<u32>()
                .unwrap(),
            from: match matches.value_of("from") {
                Some(from) => Some(from.parse::<u32>().unwrap()),
                None => None,
            },
            to: match matches.value_of("to") {
                Some(to) => Some(to.parse::<u32>().unwrap()),
                None => None,
            },
        }
    }

    fn run(&self) {
        // TODO
    }
}

impl Distinct {
    fn new(matches: &ArgMatches) -> Distinct {
        Distinct {
            input_file: matches.value_of("input_file").unwrap().to_owned(),
            from: match matches.value_of("from") {
                Some(from) => Some(from.parse::<u32>().unwrap()),
                None => None,
            },
            to: match matches.value_of("to") {
                Some(to) => Some(to.parse::<u32>().unwrap()),
                None => None,
            },
        }
    }

    fn run(&self) {
        // TODO
    }
}

fn main() {
    // TODO: forbid running with no subcommand
    // Make sure that numbers are numbers. Can clap check this?
    let matches = App::new("stat_extractor")
        .version("0.1.0")
        .author("Joris V. <joris.valette@gmail.com>")
        .about("HackerNews stat extractor")
        .arg(Arg::with_name("input_file").required(true))
        .subcommand(
            SubCommand::with_name("top")
                .arg(Arg::with_name("nb_top_queries"))
                .arg(Arg::with_name("from").long("from").takes_value(true))
                .arg(Arg::with_name("to").long("to").takes_value(true)),
        )
        .subcommand(
            SubCommand::with_name("distinct")
                .arg(Arg::with_name("from").long("from").takes_value(true))
                .arg(Arg::with_name("to").long("to").takes_value(true)),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("top") {
        let top = Top::new(matches);
        top.run();
    } else if let Some(matches) = matches.subcommand_matches("distinct") {
        let distinct = Distinct::new(matches);
        distinct.run();
    }
}
