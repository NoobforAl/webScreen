use env_logger;

pub fn init_logger(debug: bool, silent: bool) {
    let mut level = log::LevelFilter::Info;

    if debug {
        level = log::LevelFilter::Debug;
    } else if silent {
        level = log::LevelFilter::Off;
    }

    env_logger::builder().filter_level(level).init();
}
