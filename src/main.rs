use crussmap::{log::init_logger, view::view_chain};

fn main() {
    init_logger();
    let cli = Cli::parse();
    match &cli.command {
        Commands::View {
            csv,
            input,
            output,
            rewrite,
        } => view_chain(input, output, *csv, *rewrite),
    }
    // let a = read_test();
    // println!("{:?}", a)
}

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// View chain file in tsv/csv format
    View {
        /// Input chain file: *.chain/*.chain.gz supported; if not set, read from stdin
        #[arg(short, long, required = false)]
        input: Option<String>,
        /// Output file path, if not set, output to stdout
        #[arg(short, long, required = false)]
        output: Option<String>,
        /// Output csv format, default is false
        #[arg(short, long, default_value = "false", required = false)]
        csv: bool,
        /// Rewrite output file, default is false
        #[arg(short, long, default_value = "false")]
        rewrite: bool,
    },
}
