// use env_logger::Builder;
use log::LevelFilter;

pub fn init_logger(_level: LevelFilter) {
    // Builder::new().filter_level(level).init();
    env_logger::init();
}
