use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// send url website
    #[arg(short, long)]
    pub url: String,

    /// Run Debug Mode App
    #[arg(long, default_value_t = false)]
    pub debug: bool,

    /// Set Timeout for requests
    #[arg(short, long, default_value_t = 10)]
    pub timeout: u64,

    // select format png or jpeg
    #[arg(short, long, default_value = "png")]
    pub format: String,

    /// quality of image 10 - 100
    #[arg(short, long, default_value_t = 100)]
    pub quality: u32,

    /// run tools server
    #[arg(short, long, default_value_t = false)]
    pub run_server: bool,

    /// port by default is 8080
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}
