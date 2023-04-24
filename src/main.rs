use clap::{Parser, Subcommand};
use crussmap::{bed::cross_bed, log::init_logger, view::view_chain};
// use crussmap::test_ryon::test;

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
        Commands::Bed {
            bed,
            input,
            output,
            unmap,
            rewrite,
        } => cross_bed(bed, input, output, unmap, *rewrite),
    }
    // test();
}

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
        /// Input chain file: *.chain/*.chain.gz supported; if not set, read from STDIN
        #[arg(short, long, required = false)]
        input: Option<String>,
        /// Output file path, if not set, output to STDOUT
        #[arg(short, long, required = false)]
        output: Option<String>,
        /// Output in csv format, default is false
        #[arg(short, long, default_value = "false", required = false)]
        csv: bool,
        /// Rewrite output file, default is false
        #[arg(short, long, default_value = "false", required = false)]
        rewrite: bool,
    },

    /// Converts BED file. Regions mapped to multiple locations to the new assembly will be split.
    Bed {
        /// bed file path
        #[arg(short, long)]
        bed: String,
        /// input chain file path
        #[arg(short, long)]
        input: Option<String>,
        /// output bed file path, if not set, output to STDOUT
        #[arg(short, long)]
        output: Option<String>,
        /// unmapped bed file path, if not set, output to STDOUT
        #[arg(short, long)]
        unmap: Option<String>,
        /// rewrite output file, default is false
        #[arg(short, long, default_value = "false")]
        rewrite: bool,
    },
    // TODO: add test
    // TODO: Region
    // TODO: Suppprt MAF/PAF/SAM/delta -> chain
}
