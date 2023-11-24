use std::fs;
use log::LevelFilter;


pub(crate) fn setup_logger ( mod_path: &str ) -> Result<(), fern::InitError> {
    const LOG_FILE: &str = "output.log";
    let _ = fs::remove_file(LOG_FILE);  // !! ignoring possible real errors
    fern::Dispatch::new()
        .format(|out, message, _record| {
            out.finish(format_args!(
                "{}",
                // "[{}][{}] {}",
                    // "[{}]:[{}][{}] {}",
                    // humantime::format_rfc3339_seconds(SystemTime::now()),
                // record.target(),
                // record.level(),
                message
            ))
        })
        .level(LevelFilter::Error)
        .level_for(mod_path.to_owned(), LevelFilter::Debug)
        .level_for("fofdata", LevelFilter::Debug)
        // .chain(std::io::stdout())
        .chain(fern::log_file(LOG_FILE)?)
        .apply()?;
    Ok(())
}
