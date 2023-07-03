use env_logger;
pub use log::{debug, error, info, warn};

pub fn init_logger(debug: bool) {
    if debug {
        env_logger::builder()
            .filter_level(log::LevelFilter::Debug)
            .init();

        debug!("debug mode is ON!");
    } else {
        env_logger::builder()
            .filter_level(log::LevelFilter::Info)
            .init();
    }
}
