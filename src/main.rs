use clap::{App, Arg, ArgMatches, SubCommand};
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::str;
use std::u64;

fn main() {
    // TODO: forbid running with no subcommand
    // TODO: make sure that numbers are numbers. Can clap check this?
    // TODO: use radix_trie crate?
    // TODO: use errors instead of unwrap()
    // TODO: refactor if I can
    let matches = App::new("stat_extractor")
        .version("0.1.0")
        .author("Joris V. <joris.valette@gmail.com>")
        .about("HackerNews stat extractor")
        .arg(Arg::with_name("input_file").required(true))
        .subcommand(
            SubCommand::with_name("distinct")
                .arg(Arg::with_name("from").long("from").takes_value(true))
                .arg(Arg::with_name("to").long("to").takes_value(true)),
        )
        .subcommand(
            SubCommand::with_name("top")
                .arg(Arg::with_name("from").long("from").takes_value(true))
                .arg(Arg::with_name("to").long("to").takes_value(true)),
        )
        .get_matches();

    let input_file = matches.value_of("input_file").unwrap();

    if let Some(subcommand_matches) = matches.subcommand_matches("distinct") {
        let distinct = Distinct::new(input_file, subcommand_matches);
        distinct.run();
    } else if let Some(subcommand_matches) = matches.subcommand_matches("top") {
        let top = Top::new(input_file, subcommand_matches);
        top.run();
    }
}

struct Distinct {
    input_file: String,
    from: u64,
    to: u64,
}

impl Distinct {
    fn new(input_file: &str, matches: &ArgMatches) -> Distinct {
        Distinct {
            input_file: input_file.to_owned(),
            from: match matches.value_of("from") {
                Some(from) => from.parse::<u64>().unwrap(),
                None => 0,
            },
            to: match matches.value_of("to") {
                Some(to) => to.parse::<u64>().unwrap(),
                None => u64::MAX,
            },
        }
    }

    fn run(&self) {
        let file = File::open(&self.input_file).unwrap();
        let mut buf_reader = BufReader::new(file);
        let mut timestamp_buf = [0; 10];
        let mut tab = [0; 1];
        let mut queries_vec = vec![];
        loop {
            let mut query = Box::new(vec![]);
            let _ = buf_reader.read_exact(&mut timestamp_buf);
            let _ = buf_reader.read_exact(&mut tab);
            let nb_bytes_query = buf_reader.read_until(b'\n', &mut query).unwrap();
            if nb_bytes_query == 0 {
                break;
            }
            let timestamp = str::from_utf8(&timestamp_buf)
                .unwrap()
                .parse::<u64>()
                .unwrap();
            // read_until includes '\n' in query, but we don't want it
            query.truncate(query.len() - 1);
            if timestamp >= self.from && timestamp <= self.to {
                queries_vec.push(query);
            }
        }
        println!("{}", queries_vec.into_iter().unique().count());
    }
}

struct Top {
    input_file: String,
    nb_top_queries: u32,
    from: u64,
    to: u64,
}

impl Top {
    fn new(input_file: &str, matches: &ArgMatches) -> Top {
        Top {
            input_file: input_file.to_owned(),
            nb_top_queries: matches
                .value_of("nb_top_queries")
                .unwrap()
                .parse::<u32>()
                .unwrap(),
            from: match matches.value_of("from") {
                Some(from) => from.parse::<u64>().unwrap(),
                None => 0,
            },
            to: match matches.value_of("to") {
                Some(to) => to.parse::<u64>().unwrap(),
                None => u64::MAX,
            },
        }
    }

    fn run(&self) {
        // TODO
    }
}
