use std::error::Error;

#[path = "core/webscreen.rs"]
mod web_screen;

#[path = "cmd/cmd.rs"]
mod cmd;

fn main() -> Result<(), Box<dyn Error>> {
    cmd::cmd()?;
    Ok(())
}
