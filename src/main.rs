use std::process::Command;

fn main() -> color_eyre::Result<()> {
    Command::new("x") // 👈 add `.` and wait for auto-completion here
        .status()?;
    Ok(())
}
