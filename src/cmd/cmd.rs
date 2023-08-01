use clap::Parser;
use std::error::Error;

#[path = "../core/webscreen.rs"]
mod web_screen;

#[path = "../logger/logger.rs"]
mod logger;

#[path = "../api/api.rs"]
mod api;

const USAGE: &str = r#"
Usage: webscreen [OPTIONS]
    
Options:
    -u, --url <URL>          send url website [default: ]
        --silent             Run silent Mode App
        --debug              Run Debug Mode App
    -t, --timeout <TIMEOUT>  Set Timeout for requests [default: 10]
    -f, --format <FORMAT>    [default: png]
    -q, --quality <QUALITY>  quality of image 10 - 100 [default: 100]
    -r, --run-server         run tools server
    -p, --port <PORT>        port by default is 8080 [default: 8080]
    -h, --help               Print help
    -V, --version            Print version
"#;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// send url website
    #[arg(short, long, default_value = "")]
    url: String,

    /// Run silent Mode App
    #[arg(long, default_value_t = false)]
    silent: bool,

    /// Run Debug Mode App
    #[arg(long, default_value_t = false)]
    debug: bool,

    /// Set Timeout for requests
    #[arg(short, long, default_value_t = 10)]
    timeout: u64,

    // select format png or jpeg
    #[arg(short, long, default_value = "png")]
    format: String,

    /// quality of image 10 - 100
    #[arg(short, long, default_value_t = 100)]
    quality: u32,

    /// run tools server
    #[arg(short, long, default_value_t = false)]
    run_server: bool,

    /// port by default is 8080
    #[arg(short, long, default_value_t = 8080)]
    port: u16,
}

fn make_file_name(format: &str) -> String {
    return format!("./screenShot.{format}");
}

pub fn cmd() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    if !args.run_server && args.url == "" {
        println!("{}", USAGE);
        return Ok(());
    }

    logger::init_logger(args.debug, args.silent);

    if args.run_server {
        let _ = api::run(args.port);
    }

    let res = web_screen::web_screen(&args.url, args.timeout, &args.format, args.quality)?;
    web_screen::save_img(make_file_name(&args.format).as_str(), res)?;
    Ok(())
}
