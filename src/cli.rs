use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "parq-view", version, about = "Parquet file viewer (schema, head, stats, info)")]
pub struct Cli {
    #[arg(long, value_name = "PATH", global = true)]
    pub path: PathBuf,

    #[arg(long, global = true)]
    pub json: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Info,

    Schema,

    Head {
        #[arg(long, default_value_t = 20)]
        limit: usize,
        #[arg(long, value_name = "COLS", value_delimiter = ",")]
        columns: Option<Vec<String>>,
    },

    Stats,
}
