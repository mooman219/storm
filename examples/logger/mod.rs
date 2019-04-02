use log::*;

static LOGGER: SimpleLogger = SimpleLogger;

pub struct SimpleLogger;

impl SimpleLogger {
    pub fn init(level_filter: LevelFilter) {
        set_logger(&LOGGER).expect("Unable to setup logging for storm-engine.");
        set_max_level(level_filter);
    }
}

impl Log for SimpleLogger {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        println!("{:<5} {}", record.level().to_string(), record.args());
    }

    fn flush(&self) {}
}
