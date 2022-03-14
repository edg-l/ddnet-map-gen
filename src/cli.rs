use crate::generators::{fly::FlyGenerator, maze::MazeGenerator, MapGenerator};
use clap::{arg, command, Command};
use eyre::Result;
use std::path::Path;

pub fn run_cli() -> Result<()> {
    let matches = command!()
        .about("A DDraceNetwork map generator.")
        .arg_required_else_help(true)
        .subcommand_required(true)
        .arg(arg!(<FILE> "The output file").required(true))
        .subcommand(Command::new("maze").about("Generate a maze-like map."))
        .subcommand(Command::new("fly").about("Generate a map for fly techniques."))
        .get_matches();

    let output = matches.value_of("FILE").expect("output is required");
    let output = Path::new(output);

    let mut rng = rand::thread_rng();

    match matches.subcommand() {
        Some(("maze", _sub_m)) => MazeGenerator::save_file(&mut rng, output)?,
        Some(("fly", _sub_m)) => FlyGenerator::save_file(&mut rng, output)?,
        _ => panic!("invalid command"),
    }

    Ok(())
}
