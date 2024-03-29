#[macro_use]
extern crate clap;

use clap::{App, AppSettings, Arg, ArgMatches, ErrorKind, SubCommand};
use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::str;
use std::u64;

fn main() {
    // TODO: use errors instead of unwrap()
    // TODO: refactor if I can
    let matches = App::new("stat_extractor")
        .version("0.1.0")
        .author("Joris V. <joris.valette@gmail.com>")
        .about("HackerNews stat extractor")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(Arg::with_name("input_file").required(true))
        .subcommand(
            SubCommand::with_name("distinct")
                .arg(Arg::with_name("from").long("from").takes_value(true))
                .arg(Arg::with_name("to").long("to").takes_value(true)),
        )
        .subcommand(
            SubCommand::with_name("top")
                .arg(
                    Arg::with_name("nb_top_queries")
                        .takes_value(true)
                        .required(true),
                )
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
            from: value_t!(matches.value_of("from"), u64).unwrap_or_else(|e| match e.kind {
                ErrorKind::ArgumentNotFound => 0,
                _ => e.exit(),
            }),
            to: value_t!(matches.value_of("to"), u64).unwrap_or_else(|e| match e.kind {
                ErrorKind::ArgumentNotFound => u64::MAX,
                _ => e.exit(),
            }),
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
    nb_top_queries: usize,
    from: u64,
    to: u64,
}

impl Top {
    fn new(input_file: &str, matches: &ArgMatches) -> Top {
        Top {
            input_file: input_file.to_owned(),
            nb_top_queries: value_t!(matches.value_of("nb_top_queries"), usize)
                .unwrap_or_else(|e| e.exit()),
            from: value_t!(matches.value_of("from"), u64).unwrap_or_else(|e| match e.kind {
                ErrorKind::ArgumentNotFound => 0,
                _ => e.exit(),
            }),
            to: value_t!(matches.value_of("to"), u64).unwrap_or_else(|e| match e.kind {
                ErrorKind::ArgumentNotFound => u64::MAX,
                _ => e.exit(),
            }),
        }
    }

    fn run(&self) {
        let file = File::open(&self.input_file).unwrap();
        let mut buf_reader = BufReader::new(file);
        let mut timestamp_buf = [0; 10];
        let mut tab = [0; 1];
        let mut queries_map = HashMap::new();
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
                let count = match queries_map.get(&query) {
                    Some(&count) => count + 1,
                    None => 1,
                };
                queries_map.insert(query, count);
            }
        }
        let queries_map_iter = queries_map.iter().sorted_by(|a, b| b.1.cmp(a.1));
        for (key, value) in queries_map_iter.take(self.nb_top_queries) {
            println!("{} {}", value, str::from_utf8(&key).unwrap());
        }
    }
}
