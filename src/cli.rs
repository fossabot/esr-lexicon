use clap::{App, Arg, ArgMatches};

pub fn cli<'a>() -> ArgMatches<'a> {
    App::new("ESR lexicon")
        .version("1.1")
        .author("Mathis EON. <eon@abes.fr>")
        .about("Build NER dictionaries")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .conflicts_with_all(&["url", "jq", "output"])
                .takes_value(true),
        )
        .arg(
            Arg::with_name("silent")
                .short("s")
                .long("silent")
                .help("Silent output"),
        )
        .arg(
            Arg::with_name("url")
                .short("u")
                .long("url")
                .value_name("URL")
                .takes_value(true)
                .requires_all(&["jq", "output"])
                .help("Input URL"),
        )
        .arg(
            Arg::with_name("jq")
                .short("jq")
                .long("jq")
                .value_name("EXPRESSION")
                .takes_value(true)
                .requires_all(&["url", "output"])
                .help("Expression used for parsing data"),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("FILE")
                .takes_value(true)
                .requires_all(&["url", "jq"])
                .help("Output file"),
        )
        .get_matches()
}
