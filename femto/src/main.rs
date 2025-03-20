use std::{error::Error, time::SystemTime};

use femto_common::{femto_cli, fern, log};
use handlers::HandleSchema;
mod handlers;

fn setup_logger(verbosity: u8) -> Result<(), Box<dyn Error>> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .level((|v: u8| {
            match v {
                0 => log::LevelFilter::Warn,
                1 => log::LevelFilter::Info,
                2 => log::LevelFilter::Debug,
                _ => log::LevelFilter::Trace
            }
        })(verbosity))
        .chain(std::io::stdout())
        .apply()?;
    Ok(())

}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = femto_cli().get_matches();
    let verbosity = matches.get_count("verbose");
    setup_logger(verbosity)?;

    Ok(match matches.subcommand() {
        None => (),
        Some(("schema", args)) => HandleSchema::new(args).handle()?,
        _ => ()
    })
}
