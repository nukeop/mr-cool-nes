use std::collections::HashMap;
use clap::{App, Arg};

pub fn read_cl_args<'a>() -> HashMap<String, String> {
    let matches = App::new("mr-cool-nes")
        .version("0.1.0")
        .about("nes emulator")
        .author("nukeop <nukeop@gumblert.tech>")
        .arg(Arg::with_name("rom")
             .short("r")
             .long("rom")
             .value_name("FILE")
             .help("Rom to load and run"))
        .arg(Arg::with_name("config")
             .short("c")
             .long("config")
             .value_name("FILE")
             .help("Optional custom config file"))
        .get_matches();

    let rom = matches.value_of("rom").unwrap_or("rom.nes").to_owned();
    let config = matches.value_of("config").unwrap_or(".mrcoolnes").to_owned();
    let mut result: HashMap<String, String> = HashMap::new();
    result.insert("rom".to_owned(), rom);
    result.insert("config".to_owned(), config);
    result
}
