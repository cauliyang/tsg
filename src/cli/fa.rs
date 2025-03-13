use std::path::{Path, PathBuf};

use crate::graph::TSGraph;
use crate::io;
use anyhow::Result;
use tracing::info;

pub fn to_fa<P: AsRef<Path>>(input: P, output: Option<PathBuf>) -> Result<()> {
    let mut tsg_graph = TSGraph::from_file(input.as_ref())?;

    let output_path = match output {
        Some(path) => path,
        None => {
            let mut output = input.as_ref().to_path_buf();
            output.set_extension("fa");
            output
        }
    };

    info!("Writing FASTA to: {}", output_path.display());
    io::to_fa(&mut tsg_graph, &output_path)?;
    Ok(())
}
