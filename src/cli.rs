use crate::generators::{fly::FlyGenerator, maze::MazeGenerator, MapGenerator};
use clap::{arg, Parser, Subcommand};
use eyre::Result;
use owo_colors::OwoColorize;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use rand_chacha::ChaCha8Rng;
use rand_seeder::Seeder;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The width of the map.
    #[arg(long, default_value_t = 1000)]
    width: usize,
    /// The height of the map.
    #[arg(long, default_value_t = 1000)]
    height: usize,
    /// The seed used when generating a map. By default a random one.
    #[arg(short, long)]
    seed: Option<String>,
    /// The mapres directory.
    #[arg(short, long, default_value = "mapres")]
    mapres: PathBuf,
    /// The output map file.
    #[arg(short, long, default_value = "generated.map")]
    output: PathBuf,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a map for fly techniques.
    Fly {
        #[arg(long, default_value_t = 12)]
        max_fly_width: u16,
        /// The output map file.
        #[arg(long, default_value_t = 3)]
        min_fly_width: u16,
    },
    /// Generate a maze-like map.
    Maze,
}

impl Commands {
    pub fn print(&self) {
        print!("Selected generator: ");
        let data = match self {
            Self::Fly {
                max_fly_width,
                min_fly_width,
            } => format!(
                "{}, parameters: max_fly_width={}, min_fly_width={}",
                "Fly".purple().bold(),
                max_fly_width.purple(),
                min_fly_width.purple()
            ),
            Self::Maze => format!("{}", "Maze".purple().bold()),
        };
        println!("{data}");
    }
}

pub fn run_cli() -> Result<()> {
    let cli = Cli::parse();

    let seed: String = {
        if let Some(x) = &cli.seed {
            x.clone()
        } else {
            thread_rng()
                .sample_iter(&Alphanumeric)
                .take(8)
                .map(char::from)
                .collect()
        }
    };

    println!("Using seed: {}", seed.green().bold());

    let mut rng: ChaCha8Rng = Seeder::from(seed).make_rng();

    cli.command.print();

    match cli.command {
        Commands::Maze => {
            MazeGenerator.save_file(&mut rng, &cli.mapres, cli.width, cli.height, &cli.output)
        }
        Commands::Fly {
            max_fly_width,
            min_fly_width,
        } => FlyGenerator {
            max_fly_width,
            min_fly_width,
        }
        .save_file(&mut rng, &cli.mapres, cli.width, cli.height, &cli.output),
    }
}
