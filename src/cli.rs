use crate::generators::{fly::FlyGenerator, maze::MazeGenerator, MapGenerator};
use clap::{arg, command, Command};
use eyre::Result;
use std::path::Path;

pub fn run_cli() -> Result<()> {
    let matches = command!()
        .about("A DDraceNetwork map generator")
        .arg_required_else_help(true)
        .subcommand_required(true)
        .arg(arg!(<FILE> "The output map file").required(true))
        .subcommand(
            Command::new("maze")
                .about("Generate a maze-like map")
                .arg(
                    arg!(--width <WIDTH> "The width of the map")
                        .default_value("1000")
                        .required(false),
                )
                .arg(
                    arg!(--height <HEIGHT> "The height of the map")
                        .default_value("1000")
                        .required(false),
                ),
        )
        .subcommand(
            Command::new("fly")
                .about("Generate a map for fly techniques")
                .arg(
                    arg!(--width <WIDTH> "The width of the map")
                        .default_value("1000")
                        .required(false),
                )
                .arg(
                    arg!(--height <HEIGHT> "The height of the map")
                        .default_value("1000")
                        .required(false),
                ),
        )
        .get_matches();

    let output = matches.value_of("FILE").expect("output is required");
    let output = Path::new(output);

    let mut rng = rand::thread_rng();

    match matches.subcommand() {
        Some(("maze", sub_m)) => {
            let width: usize = sub_m.value_of_t("width").unwrap_or_else(|e| e.exit());
            let height: usize = sub_m.value_of_t("height").unwrap_or_else(|e| e.exit());
            MazeGenerator::save_file(&mut rng, width, height, output)?
        }
        Some(("fly", sub_m)) => {
            let width: usize = sub_m.value_of_t("width").unwrap_or_else(|e| e.exit());
            let height: usize = sub_m.value_of_t("height").unwrap_or_else(|e| e.exit());
            FlyGenerator::save_file(&mut rng, width, height, output)?
        }
        _ => panic!("invalid command"),
    }

    Ok(())
}
