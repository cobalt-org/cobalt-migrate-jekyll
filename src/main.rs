#![warn(warnings)]

extern crate cobalt;
extern crate cobalt_migrate_jekyll;
extern crate env_logger;

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate clap;

extern crate log;

use std::io::Write;
use std::path;

use clap::App;
use cobalt::error::*;

quick_main!(run);

fn run() -> Result<()> {
    let app_cli = App::new("cobalt-migrate-jekyll")
        .version(crate_version!())
        .about("Migration from Jekyll to cobalt.rs.")
        .args(&get_logging_args())
        .arg(
            clap::Arg::with_name("source")
                .long("source")
                .value_name("JEKYLL-FILE-OR-DIR")
                .help("Jekyll posts' directory")
                .required(true)
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("destination")
                .long("destination")
                .value_name("DIR")
                .help("Output dir of converted posts")
                .takes_value(true)
                .default_value("./posts"),
        );

    let global_matches = app_cli.get_matches();

    let mut builder = get_logging(&global_matches)?;
    builder.init();

    convert_command(&global_matches)?;

    Ok(())
}

pub fn get_logging_args() -> Vec<clap::Arg<'static, 'static>> {
    [
        clap::Arg::with_name("log-level")
            .short("L")
            .long("log-level")
            .possible_values(&["error", "warn", "info", "debug", "trace", "off"])
            .help("Log level [default: info]")
            .global(true)
            .takes_value(true),
        clap::Arg::with_name("trace")
            .long("trace")
            .help("Log ultra-verbose (trace level) information")
            .global(true)
            .takes_value(false),
        clap::Arg::with_name("silent")
            .long("silent")
            .help("Suppress all output")
            .global(true)
            .takes_value(false),
    ].to_vec()
}

pub fn get_logging(matches: &clap::ArgMatches) -> Result<env_logger::Builder> {
    let mut builder = env_logger::Builder::new();

    let level = if matches.is_present("trace") {
        log::LevelFilter::Trace
    } else if matches.is_present("silent") {
        log::LevelFilter::Off
    } else {
        match matches.value_of("log-level") {
            Some("error") => log::LevelFilter::Error,
            Some("warn") => log::LevelFilter::Warn,
            Some("debug") => log::LevelFilter::Debug,
            Some("trace") => log::LevelFilter::Trace,
            Some("off") => log::LevelFilter::Off,
            Some("info") | _ => log::LevelFilter::Info,
        }
    };
    builder.filter(None, level);

    if level == log::LevelFilter::Trace {
        builder.default_format_timestamp(false);
    } else {
        builder.format(|f, record| {
            writeln!(
                f,
                "[{}] {}",
                record.level().to_string().to_lowercase(),
                record.args()
            )
        });
    }

    Ok(builder)
}

pub fn convert_command(matches: &clap::ArgMatches) -> Result<()> {
    let source = matches.value_of("source").unwrap().to_string();
    let dest = matches.value_of("destination").unwrap().to_string();

    cobalt_migrate_jekyll::convert_from_jk(path::Path::new(&source), path::Path::new(&dest))
        .chain_err(|| "Jekyll conversion failed.")
}
