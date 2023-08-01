use std::error::Error;

#[path = "cmd/cmd.rs"]
mod cmd;

fn main() -> Result<(), Box<dyn Error>> {
    cmd::cmd()?;
    Ok(())
}
