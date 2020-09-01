use simplelog::{CombinedLogger, Config, LevelFilter, WriteLogger};
use std::fs::File;
use std::process::exit;

pub fn init() {
    CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Trace,
        Config::default(),
        File::create("bot.log").unwrap(),
    )])
    .unwrap();
}

pub fn info(s: &str) {
    info!("{}", s);
}

pub fn error(s: &str) {
    error!("{}", s);
}

pub fn abort(s: &str) -> ! {
    error(s);
    exit(1);
}
