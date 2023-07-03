use clap::Parser;
use std::error::Error;

#[path = "core/webscreen.rs"]
mod web_screen;

#[path = "cmd/cmd.rs"]
mod cmd;

fn make_file_name(format: &str) -> String {
    return format!("{}{}", "./screenShot.", format);
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = cmd::Cli::parse();

    let res = web_screen::web_screen(&args.url, args.timeout, &args.format, args.quality)?;
    web_screen::save_img(make_file_name(&args.format).as_str(), res)?;
    Ok(())
}
