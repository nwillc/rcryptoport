use clap::{App, Arg, ArgMatches};

pub(crate) const CONFIG: &str = "config";
pub(crate) const DRY_RUN: &str = "dry-run";
pub(crate) const LOOP: &str = "loop";
pub(crate) const SETUP: &str = "setup";
pub(crate) const QUOTED: &str = "quoted";

pub fn cli<'a>() -> ArgMatches<'a> {
    return App::new("rcryptoport")
        .version("3.0.0")
        .author("nwillc@gmail.com")
        .about("Retrieve current value of your crypto portfolio.")
        .arg(
            Arg::with_name(CONFIG)
                .short("c")
                .long(CONFIG)
                .takes_value(true)
                .value_name("FILE")
                .help("Path to specific config file"),
        )
        .arg(
            Arg::with_name(DRY_RUN)
                .short("d")
                .long(DRY_RUN)
                .help("Dry run, do not save values"),
        )
        .arg(
            Arg::with_name(LOOP)
                .short("l")
                .long(LOOP)
                .takes_value(true)
                .value_name("SECONDS")
                .help("Run looping every SECONDS seconds"),
        )
        .arg(
            Arg::with_name(QUOTED)
                .short("q")
                .long(QUOTED)
                .takes_value(true)
                .value_name("CURRENCY")
                .help("Quote USD prices converted to CURRENCY"),
        )
        .subcommand(App::new(SETUP).about("Set up portfolio configuration"))
        .get_matches();
}
