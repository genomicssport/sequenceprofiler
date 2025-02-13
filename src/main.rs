mod args;
mod filestruct;
mod simfilter;
mod similarity;
use crate::args::CommandParse;
use crate::args::Commands;
use crate::simfilter::simfilterarg;
use crate::similarity::profilesimilarity;
use clap::Parser;

/*
  Author Gaurav Sablok
  SLB Potsdam
  Date: 2025-2-6
* */

fn main() {
    let argsparse = CommandParse::parse();
    match &argsparse.command {
        Commands::Sequence {
            sequencepath,
            sequencekmer,
        } => {
            let command = profilesimilarity(sequencepath, sequencekmer);
            println!(
                "The sequence similarity and the clustering of the sequences based on the kmer means have been written: {:?}",
                command
            );
        }
        Commands::Filter {
            sequence,
            kmer,
            threshold,
        } => {
            let command = simfilterarg(sequence, kmer, threshold).unwrap();
            println!("The filtered files have been written: {:?}", command);
        }
    }
}
