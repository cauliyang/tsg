mod cli;
mod graph;
mod io;

use tracing::info;

use anyhow::Result;
use clap::Parser;
use cli::Commands;
use graph::TSGraph;

#[derive(Parser)]
#[command(author, version, about = "Transcript Segment Graph (TSG) CLI tool")]
struct Cli {
    /// Sets the level of verbosity
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
    #[command(subcommand)]
    command: Commands,
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    // Set verbosity level
    match cli.verbose {
        0 => tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .init(),
        1 => tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init(),
        _ => tracing_subscriber::fmt()
            .with_max_level(tracing::Level::TRACE)
            .init(),
    }

    match cli.command {
        Commands::Parse { input } => {
            info!("Parsing TSG file: {}", input.display());
            let graph = TSGraph::from_file(input)?;
            info!(
                "Successfully parsed TSG file with {} nodes and {} edges",
                graph.get_nodes().len(),
                graph.get_edges().len()
            );
            Ok(())
        }
        Commands::Dot { input, output } => {
            cli::to_dot(input, output)?;
            Ok(())
        }

        Commands::Traverse { input, output } => {
            info!("Finding paths in TSG file: {}", input.display());
            let graph = TSGraph::from_file(input)?;
            let paths = graph.traverse()?;

            info!("Found {} paths", paths.len());

            if let Some(output_path) = output {
                let mut content = String::new();
                for path in paths {
                    content.push_str(&format!("{}\n", path));
                }
                std::fs::write(&output_path, content)?;
                info!("Paths written to: {}", output_path.display());
            } else {
                for path in paths {
                    println!("{}", path);
                }
            }
            Ok(())
        }
    }
}

fn main() -> Result<()> {
    run()?;
    Ok(())
}
