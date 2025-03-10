use std::path::{Path, PathBuf};

use crate::graph::TSGraph;
use crate::io;
use anyhow::Result;
use tracing::info;

pub fn to_gtf<P: AsRef<Path>>(input: P, output: Option<PathBuf>) -> Result<()> {
    let tsg_graph = TSGraph::from_file(input.as_ref())?;
    let output_path = match output {
        Some(path) => path,
        None => {
            let mut output = input.as_ref().to_path_buf();
            output.set_extension("gtf");
            output
        }
    };

    info!("Writing GTF to: {}", output_path.display());
    io::to_gtf(&tsg_graph, output_path)?;
    Ok(())
}
