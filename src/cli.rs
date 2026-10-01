use std::path::PathBuf;

use clap::{ArgGroup, Parser, Subcommand};

#[derive(Parser, Debug)]
// Define a group named "input". It requires exactly ONE of the fields to be present.
#[command(group(
    ArgGroup::new("input")
        .args(["positional_input", "named_input"])
        .multiple(false),
))]
pub struct Cli {
    positional_input: Option<PathBuf>,

    /// Pass the input file as a named argument
    // (e.g., `program --input input.txt` or `-i input.txt`)
    #[arg(short, long = "input")]
    named_input: Option<PathBuf>,

    /// The video directory for the files to be converted
    #[arg(short, long, required = false)]
    pub video_dir: Option<String>,
    /// The output directory for the converted files
    #[arg(short, long, required = false)]
    pub output_dir: Option<String>,

    #[arg(short = 'r', long, required = false)]
    pub overwrite: bool,
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Config {
        #[arg(short = 'p', long, required = false)]
        show_path: bool,
    },
}

pub struct CliArgs {
    pub input: PathBuf,
    pub video_dir: Option<PathBuf>,
    pub output_dir: Option<PathBuf>,
    pub overwrite: bool,
}

impl Cli {
    pub fn get_args() -> CliArgs {
        let cli = Cli::parse();

        if let Some(command) = cli.command {
            let app_dir = dirs::config_dir().unwrap();
            let config_file_path = app_dir.join("ascii").join("config.json");

            match command {
                Commands::Config { show_path } => {
                    if show_path {
                        println!("{}", config_file_path.display());
                    }
                }
            }

            std::process::exit(0);
        }

        // Normal program execution: input IS required.
        let input = match (cli.positional_input, cli.named_input) {
            (Some(input), None) | (None, Some(input)) => input,
            (Some(_), Some(_)) => {
                // This normally gets caught by ArgGroup if you specify conflicts.
                unreachable!()
            }
            (None, None) => {
                eprintln!("error: input file is required");
                std::process::exit(2);
            }
        };

        CliArgs {
            input,
            video_dir: cli.video_dir.map(PathBuf::from),
            output_dir: cli.output_dir.map(PathBuf::from),
            overwrite: cli.overwrite,
        }
    }
}
