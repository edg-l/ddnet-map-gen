use color_eyre::Result;

mod cli;
pub mod generators;

fn main() -> Result<()> {
    color_eyre::install()?;
    cli::run_cli()?;
    Ok(())
}
