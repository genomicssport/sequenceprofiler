use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(name = "graph-kmers", version = "1.0", about = "graphkmers")]
pub struct CommandParse {
    /// subcommands for the specific actions
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// identity kmer similarity index
    Sequence {
        /// provide the path to sequence file
        sequencepath: String,
        /// provide the kmer to be profiled for the sequence similarity
        sequencekmer: String,
    },
    /// identity kmer filter
    Filter {
        /// provide the path to the sequence file
        sequence: String,
        /// sequence kmer for the identity kmer
        kmer: String,
        /// provide the threshold
        threshold: String,
    },
}
